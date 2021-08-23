use std::{
    collections::HashSet,
    sync::{mpsc::channel, Arc},
};

use crate::*;

struct SystemBeingScheduled {
    need_to_wake_up: HashSet<usize>,
    waiting_on_count: usize,
}

struct ResourceAccessGroup {
    /// If this group of systems requires mutable access to the archetype channel.
    mutable: bool,
    /// If this is waiting on a mutable system, which mutable system?
    waiting_on_index: usize,
    /// Which systems are in this group.
    /// For a read operation this may be any number, for a write system this will always be 1 system.
    systems: Vec<usize>,
}

struct SubSchedule {
    exclusive_system: Option<usize>,
    systems_being_scheduled: Vec<SystemBeingScheduled>,
    /// Key is archetype index and channel
    /// Value is an index into systems
    resources: HashMap<(usize, usize), ResourceAccessGroup>,
    last_exclusive_system_index: usize,
    systems_to_schedule: Vec<usize>,
}

impl SubSchedule {
    pub fn new(exclusive_system: Option<usize>) -> Self {
        Self {
            exclusive_system,
            // The first system is the one that starts the whole thing, but it waits on nothing.
            systems_being_scheduled: vec![SystemBeingScheduled {
                need_to_wake_up: HashSet::new(),
                waiting_on_count: 0,
            }],
            resources: HashMap::new(),
            last_exclusive_system_index: 0,
            systems_to_schedule: Vec::new(),
        }
    }

    /// The `system_index` refers to the index within systems passed in during `schedule`.
    pub fn add_system(&mut self, system_index: usize) {
        // Add 1 because there's always a default system.
        self.systems_to_schedule.push(system_index);
    }

    pub fn schedule(&mut self, world: &World, systems: &Vec<System>) {
        for system_index in self.systems_to_schedule.drain(..) {
            let system = &systems[system_index];
            let new_system_index = self.systems_being_scheduled.len();

            let mut waiting_on_count = 0;

            match &system.function {
                SystemFunction::Exclusive(_) => {
                    // Exclusive systems create a new sub-schedule.
                    for current_systems in self.resources.values_mut() {
                        for system_index in &current_systems.systems {
                            if self.systems_being_scheduled[*system_index]
                                .need_to_wake_up
                                .insert(new_system_index)
                            {
                                waiting_on_count += 1;
                            }
                        }
                        current_systems.systems.clear();
                        current_systems.systems.push(new_system_index);
                        current_systems.mutable = true;
                    }
                    self.last_exclusive_system_index = new_system_index;
                }
                SystemFunction::NonExclusive { meta_data, .. } => {
                    let archetype_access = meta_data(world).unwrap();
                    for archetype_access in archetype_access {
                        /*
                        println!(
                            "ARCHETYPE ACCESS: {:?}",
                            archetype_access.archetype_index_and_channel
                        );
                        println!("MUTABLE: {:?}", archetype_access.mutable);
                        */
                        let last_exclusive_system_index = self.last_exclusive_system_index;
                        let current_systems = self
                            .resources
                            .entry((
                                archetype_access.archetype_index,
                                archetype_access.channel_index,
                            ))
                            .or_insert_with(|| ResourceAccessGroup {
                                waiting_on_index: 0,
                                mutable: true,
                                systems: vec![last_exclusive_system_index],
                            });

                        if archetype_access.mutable {
                            for system_index in &current_systems.systems {
                                // Check that we're not already waiting on this system.
                                if self.systems_being_scheduled[*system_index]
                                    .need_to_wake_up
                                    .insert(new_system_index)
                                {
                                    // println!("{:?} will wake {:?}", system_index, new_system_index);
                                    waiting_on_count += 1;
                                }
                            }

                            current_systems.systems.clear();
                            current_systems.systems.push(new_system_index);
                            current_systems.mutable = true;
                        } else {
                            if current_systems.mutable {
                                // There should only be one of these.
                                for system_index in &current_systems.systems {
                                    // Check that we're not already waiting on this system.
                                    if self.systems_being_scheduled[*system_index]
                                        .need_to_wake_up
                                        .insert(new_system_index)
                                    {
                                        // println!("{:?} will wake {:?}", system_index, new_system_index);

                                        waiting_on_count += 1;
                                        current_systems.waiting_on_index = *system_index;
                                    }
                                }
                                current_systems.systems.clear();
                                current_systems.systems.push(new_system_index);
                            } else {
                                // Check that we're not already waiting on this system.
                                if self.systems_being_scheduled[current_systems.waiting_on_index]
                                    .need_to_wake_up
                                    .insert(new_system_index)
                                {
                                    /*
                                    println!(
                                        "{:?} will wake {:?}",
                                        current_systems.waiting_on_index, new_system_index
                                    );
                                    */

                                    waiting_on_count += 1;
                                }
                                current_systems.systems.push(new_system_index);
                            }
                            current_systems.mutable = false;
                        }
                    }
                }
            }

            self.systems_being_scheduled.push(SystemBeingScheduled {
                need_to_wake_up: HashSet::new(),
                waiting_on_count,
            });
        }
    }

    #[cfg(test)]
    /// Returns schedule groups for verification in testing.
    fn generate_schedule(&self) -> Vec<Vec<usize>> {
        let mut schedule = Vec::new();

        let mut counts: Vec<_> = self
            .systems_being_scheduled
            .iter()
            .map(|s| Some(s.waiting_on_count))
            .collect();

        let mut systems_to_run_next: Vec<usize> = vec![0];
        let mut systems_ran = 0;
        loop {
            let mut group = Vec::new();

            for system_to_run in systems_to_run_next.drain(..) {
                for need_to_wake in self.systems_being_scheduled[system_to_run]
                    .need_to_wake_up
                    .iter()
                {
                    *counts[*need_to_wake].as_mut().unwrap() -= 1;
                }
                systems_ran += 1;
                counts[system_to_run] = None;
                group.push(system_to_run);
            }
            for (index, count) in counts.iter_mut().enumerate() {
                if *count == Some(0) {
                    systems_to_run_next.push(index);
                }
            }

            schedule.push(group);
            if systems_ran == counts.len() {
                break;
            }
        }
        schedule
    }
}

pub struct Scheduler {
    sub_schedules: Vec<SubSchedule>,
    systems: Vec<System>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            sub_schedules: vec![SubSchedule::new(None)],
            systems: Vec::new(),
        }
    }
    pub fn add_system<PARAMS, S: IntoSystemTrait<PARAMS>>(
        &mut self,
        system: S,
    ) -> Result<(), KecsError> {
        let system = system.system()?;
        let system_index = self.systems.len();
        match &system.function {
            SystemFunction::Exclusive(_) => {
                // Todo: Also need to hook up waking up the next schedule somehow
                self.sub_schedules
                    .push(SubSchedule::new(Some(system_index)));
            }
            SystemFunction::NonExclusive { .. } => {
                self.sub_schedules
                    .last_mut()
                    .unwrap()
                    .add_system(system_index);
            }
        }
        self.systems.push(system);

        Ok(())
    }

    fn schedule(&mut self, world: &World) {
        for sub_schedule in &mut self.sub_schedules {
            sub_schedule.schedule(world, &self.systems)
        }
    }

    pub fn run(&mut self, world: World) -> World {
        let sub_schedules = std::mem::take(&mut self.sub_schedules);
        ktasks::spawn(async move {});
        world
    }
}

enum SubSchedulerMessage {
    SystemFinished((usize, System)),
}

#[test]
fn schedule0() {
    struct A;
    impl ComponentTrait for A {}
    struct B;
    impl ComponentTrait for B {}

    let mut world = World::new();
    world.spawn(A);

    let mut scheduler = Scheduler::new();
    scheduler.add_system(|_: &A| {}).unwrap();
    scheduler.add_system(|_: &A| {}).unwrap();
    scheduler.add_system(|_: &mut A| {}).unwrap();
    scheduler.add_system(|_: &mut A| {}).unwrap();
    scheduler.schedule(&world);

    let schedule = scheduler.sub_schedules[0].generate_schedule();
    println!("{:#?}", schedule);
    assert!(schedule == vec![vec![0,], vec![1, 2,], vec![3,], vec![4,],]);
}

#[test]
fn schedule1() {
    struct A;
    impl ComponentTrait for A {}
    struct B;
    impl ComponentTrait for B {}

    let mut world = World::new();
    world.spawn(A);
    world.spawn(B);
    world.spawn((A, B));

    let mut scheduler = Scheduler::new();
    scheduler.add_system(|_: Query<(&A,)>| {}).unwrap(); // 1
    scheduler.add_system(|_: Query<(&A, &mut B)>| {}).unwrap(); // 2
    scheduler.add_system(|_: Query<(&mut B,)>| {}).unwrap(); // 3
    scheduler.schedule(&world);

    let schedule = scheduler.sub_schedules[0].generate_schedule();
    println!("{:#?}", schedule);
    assert!(schedule == vec![vec![0,], vec![1, 2,], vec![3,],])
}

#[test]
fn schedule_exclusive0() {
    struct A;
    impl ComponentTrait for A {}
    struct B;
    impl ComponentTrait for B {}

    let mut world = World::new();
    world.spawn(A);
    world.spawn(B);

    let mut scheduler = Scheduler::new();
    scheduler.add_system(|_: Query<(&A,)>| {}).unwrap(); // 1
    scheduler.add_system(|_: Query<(&A,)>| {}).unwrap(); // 2
    scheduler.add_system(|_: &mut World| {}).unwrap(); // 3
    scheduler.add_system(|_: Query<(&A,)>| {}).unwrap(); // 4
    scheduler.add_system(|_: Query<(&mut B,)>| {}).unwrap(); // 5

    scheduler.schedule(&world);
    let schedule0 = scheduler.sub_schedules[0].generate_schedule();
    let schedule1 = scheduler.sub_schedules[1].generate_schedule();

    println!("{:#?}", schedule0);
    // This demonstrates how extra parallelization can sometimes be gained
    // because systems are disjoint within a group.
    assert!(schedule0 == vec![vec![0,], vec![1, 2,]]);
    assert!(schedule1 == vec![vec![0,], vec![1, 2]]);
}

#[test]
fn run_schedule0() {
    struct A;
    impl ComponentTrait for A {}
    struct B;
    impl ComponentTrait for B {}

    let mut world = World::new();
    world.spawn(A);

    let mut scheduler = Scheduler::new();
    scheduler.add_system(|_: &A| {}).unwrap();
    scheduler.add_system(|_: &A| {}).unwrap();
    scheduler.add_system(|_: &mut A| {}).unwrap();
    scheduler.add_system(|_: &mut A| {}).unwrap();
    scheduler.schedule(&world);

    ktasks::create_workers(3);
}

/*

for mut sub_schedule in &mut self.sub_schedules {
    if let Some(exclusive_system) = sub_schedule.exclusive_system {
        match exclusive_system.function {
            SystemFunction::Exclusive(mut system) => system(self.world).unwrap(),
            _ => unreachable!(),
        }
    }
    sub_schedule.schedule();
    let (sender, receiver) = channel::<SchedulerMessage>();

    let join_handle = std::thread::spawn(move || {
        while let Ok(message) = receiver.recv() {
            match message {
                SchedulerMessage::SystemDone(system) => {
                    let mut need_to_wake_up = HashSet::new();
                    std::mem::swap(
                        &mut need_to_wake_up,
                        &mut sub_schedule.systems[system].need_to_wake_up,
                    );
                    for waiting_system in need_to_wake_up {
                        sub_schedule.systems[waiting_system].waiting_on_count -= 1;
                        if sub_schedule.systems[waiting_system].waiting_on_count == 0 {
                            // Schedule this as a new task.
                            let sender = sender.clone();
                            ktasks::spawn(async move {
                                println!("RUNNING TASK: {:?}", waiting_system);
                                sender.send(SchedulerMessage::SystemDone(waiting_system))
                            });
                        }
                    }
                }
            }
        }
    });

    join_handle.join().unwrap();
}
*/
