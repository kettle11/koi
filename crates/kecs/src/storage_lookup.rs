use crate::sparse_set::*;
use crate::*;

#[derive(Clone, Copy, Debug)]
pub enum FilterType {
    With,
    Without,
    Optional,
}

#[derive(Clone, Copy, Debug)]
pub struct Filter {
    pub component_id: ComponentId,
    pub filter_type: FilterType,
}
pub(crate) struct StorageLookup {
    // The SparseSet value contains the channel of the component.
    component_archetypes: HashMap<ComponentId, SparseSet<usize>>,
    all_archetypes: Vec<usize>,
}

impl StorageLookup {
    pub(crate) fn new() -> Self {
        Self {
            component_archetypes: HashMap::new(),
            all_archetypes: Vec::new(),
        }
    }

    pub(crate) fn new_archetype(&mut self, archetype_index: usize, component_ids: &[ComponentId]) {
        for (i, component) in component_ids.iter().enumerate() {
            self.component_archetypes
                .entry(*component)
                .or_insert_with(SparseSet::new)
                .insert(archetype_index, i)
        }
        self.all_archetypes.push(archetype_index);
    }

    pub(crate) fn matching_archetype_iterator<const CHANNEL_COUNT: usize>(
        &self,
        // Filter and an optional channel associated with the filter.
        filters: &[(Option<usize>, Filter)],
    ) -> MatchingArchetypeIterator<CHANNEL_COUNT> {
        let mut filter_info = Vec::with_capacity(filters.len());
        for (output_index, filter) in filters.iter() {
            let archetypes = self.component_archetypes.get(&filter.component_id);
            let matching_archetypes_len = match filter.filter_type {
                FilterType::With => archetypes.map_or(0, |a| a.len()),
                FilterType::Without => {
                    self.all_archetypes.len() - archetypes.map_or(0, |a| a.len())
                }
                FilterType::Optional => self.all_archetypes.len(),
            };
            filter_info.push(FilterInfo {
                archetypes,
                matching_archetypes_len,
                output_index: *output_index,
                filter_type: filter.filter_type,
            });
        }
        // Find the smallest requirement.
        filter_info.sort_by_key(|filter_info| filter_info.matching_archetypes_len);

        MatchingArchetypeIterator {
            offset: 0,
            filter_info,
        }
    }
}

#[derive(Clone, Copy)]
struct FilterInfo<'a> {
    archetypes: Option<&'a SparseSet<usize>>,
    filter_type: FilterType,
    matching_archetypes_len: usize,
    output_index: Option<usize>,
}

#[derive(Clone, Debug)]
pub(crate) struct ArchetypeMatch<const CHANNEL_COUNT: usize> {
    pub archetype_index: usize,
    /// A channel will be [None] if this [Archetype] does not contain an optional channel.
    pub channels: [Option<usize>; CHANNEL_COUNT],
}

pub(crate) struct MatchingArchetypeIterator<'a, const CHANNEL_COUNT: usize> {
    offset: usize,
    filter_info: Vec<FilterInfo<'a>>,
}

impl<'a, const CHANNEL_COUNT: usize> Iterator for MatchingArchetypeIterator<'a, CHANNEL_COUNT> {
    type Item = ArchetypeMatch<CHANNEL_COUNT>;
    fn next(&mut self) -> Option<Self::Item> {
        fn match_tail_filters(
            filters: &[FilterInfo],
            archetype_index: usize,
            channels: &mut [Option<usize>],
        ) -> bool {
            for filter_info in filters.iter() {
                match filter_info.filter_type {
                    FilterType::With => {
                        if let Some(archetypes) = filter_info.archetypes {
                            if let Some(channel) = archetypes.get(archetype_index) {
                                if let Some(output_index) = filter_info.output_index {
                                    channels[output_index] = Some(*channel);
                                }
                                continue;
                            }
                        }
                        return false;
                    }
                    FilterType::Without => {
                        if filter_info
                            .archetypes
                            .map_or(false, |a| a.get(archetype_index).is_some())
                        {
                            return false;
                        }
                    }
                    FilterType::Optional => {
                        if let Some(channel) = filter_info
                            .archetypes
                            .map(|a| a.get(archetype_index))
                            .flatten()
                        {
                            if let Some(output_index) = filter_info.output_index {
                                channels[output_index] = Some(*channel);
                            }
                        } else {
                            if let Some(output_index) = filter_info.output_index {
                                channels[output_index] = None;
                            }
                        }
                    }
                };
            }
            true
        }

        let (first_filter_info, tail_filter_info) = self.filter_info.split_first()?;

        match first_filter_info.filter_type {
            // This will almost always be the case chosen, but the other cases are handled just-in-case.
            FilterType::With => {
                let archetypes = first_filter_info.archetypes?;
                for (channel, archetype_index) in archetypes.data()[self.offset..]
                    .iter()
                    .zip(archetypes.data_index_to_item_index()[self.offset..].iter())
                {
                    // Increment offset so that the iterator can resume where it left off.
                    self.offset += 1;

                    let mut channels = [None; CHANNEL_COUNT];
                    if let Some(output_index) = first_filter_info.output_index {
                        channels[output_index] = Some(*channel);
                    }

                    if match_tail_filters(tail_filter_info, *archetype_index, &mut channels) {
                        return Some(ArchetypeMatch {
                            archetype_index: *archetype_index,
                            channels,
                        });
                    }
                }
            }
            FilterType::Without => {
                todo!()
            }
            FilterType::Optional => {
                // This case will only be reached if all [Filter]s are `Optional`.
                // In this case all [Archetype]s match and need to be iterated
                todo!()
            }
        };

        None
    }
}
