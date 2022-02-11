use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{mpsc, Arc, Weak};

struct Item<T> {
    item: T,
    indirect_index: usize,
}

struct IndirectionStorage<T> {
    items: Vec<Item<T>>,
    indirect_indices: Vec<usize>,
    free_indirect_indices: Vec<usize>,
}
impl<T> IndirectionStorage<T> {
    fn new() -> Self {
        Self {
            items: Vec::new(),
            indirect_indices: Vec::new(),
            free_indirect_indices: Vec::new(),
        }
    }

    /// Push an item and return its indirect index.
    fn push(&mut self, item: T) -> usize {
        let indirect_index = if let Some(indirect_index) = self.free_indirect_indices.pop() {
            self.indirect_indices[indirect_index] = self.items.len();
            indirect_index
        } else {
            self.indirect_indices.push(self.items.len());
            self.indirect_indices.len() - 1
        };
        self.items.push(Item {
            item,
            indirect_index,
        });
        indirect_index
    }

    fn remove(&mut self, indirect_index: usize) -> T {
        let item_index = self.indirect_indices[indirect_index];
        self.indirect_indices[self.items.last().unwrap().indirect_index] = item_index;
        let old_item = self.items.swap_remove(item_index);
        self.free_indirect_indices.push(indirect_index);
        old_item.item
    }

    /// Gets an indirection_index without actually pushing an item.
    /// The indirection_index will be set to point to 0, which
    /// is presumably the default.
    fn get_new_indirection_index(&mut self) -> usize {
        if let Some(indirect_index) = self.free_indirect_indices.pop() {
            self.indirect_indices[indirect_index] = 0;
            indirect_index
        } else {
            self.indirect_indices.push(0);
            self.indirect_indices.len() - 1
        }
    }

    fn replace_placeholder(&mut self, indirect_index: usize, item: T) {
        self.items.push(Item {
            item,
            indirect_index,
        });
        debug_assert!(self.indirect_indices[indirect_index] == 0);
        self.indirect_indices[indirect_index] = self.items.len() - 1;
    }

    fn get(&self, indirect_index: usize) -> &T {
        &self.items[self.indirect_indices[indirect_index]].item
    }

    fn get_mut(&mut self, indirect_index: usize) -> &mut T {
        &mut self.items[self.indirect_indices[indirect_index]].item
    }

    fn is_placeholder(&self, indirect_index: usize) -> bool {
        self.indirect_indices[indirect_index] == 0
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

pub struct Handle<T> {
    indirection_index: usize,
    drop_handle: Option<Arc<DropHandle>>,
    phantom: std::marker::PhantomData<T>,
}

unsafe impl<T> Send for Handle<T> {}
unsafe impl<T> Sync for Handle<T> {}

use kecs::*;
impl<T: 'static> ComponentTrait for Handle<T> {
    fn clone_components(
        _entity_migrator: &mut EntityMigrator,
        items: &[Self],
    ) -> Option<Vec<Self>> {
        Some(items.into())
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            indirection_index: self.indirection_index,
            drop_handle: self.drop_handle.clone(),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T> Default for Handle<T> {
    /// Creates a new handle that references whatever the default for this asset is.
    fn default() -> Self {
        Self {
            indirection_index: 0,
            drop_handle: None,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.indirection_index == other.indirection_index
    }
}

impl<T> Eq for Handle<T> {}

impl<T> PartialOrd for Handle<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Handle<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.indirection_index.cmp(&other.indirection_index)
    }
}

impl<T> Debug for Handle<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Handle")
            .field("index", &self.indirection_index)
            .finish()
    }
}

impl<T> Handle<T> {
    fn new(indirection_index: usize, channel: mpsc::Sender<usize>) -> Self {
        Self {
            indirection_index,
            drop_handle: Some(Arc::new(DropHandle {
                indirection_index,
                channel: SyncGuard::new(channel),
            })),
            phantom: std::marker::PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) const fn new_with_just_index(indirection_index: usize) -> Self {
        Self {
            indirection_index,
            drop_handle: None,
            phantom: std::marker::PhantomData,
        }
    }

    fn clone_weak(&self) -> WeakHandle<T> {
        WeakHandle {
            indirection_index: self.indirection_index,
            drop_handle: self.drop_handle.as_ref().map(Arc::downgrade),
            phantom: std::marker::PhantomData,
        }
    }
}

pub struct DropHandle {
    indirection_index: usize,
    channel: SyncGuard<mpsc::Sender<usize>>,
}

impl Drop for DropHandle {
    fn drop(&mut self) {
        let _ = self.channel.inner().send(self.indirection_index);
    }
}

struct WeakHandle<T> {
    indirection_index: usize,
    drop_handle: Option<Weak<DropHandle>>,
    phantom: std::marker::PhantomData<T>,
}

impl<T> WeakHandle<T> {
    /// Upgrades this [WeakHandle<T>] to a full [Handle<T>]
    /// This will return [None] if all [Handle<T>]s have already been dropped.
    pub fn upgrade(&self) -> Option<Handle<T>> {
        Some(Handle {
            drop_handle: if let Some(drop_handle) = &self.drop_handle {
                Some(drop_handle.upgrade()?)
            } else {
                None
            },
            indirection_index: self.indirection_index,
            phantom: std::marker::PhantomData,
        })
    }
}

/// [Assets] keeps track of assets and enables loading
/// ref-counted [Handle]s to different types of assets.
/// Many types of [Assets] are used by koi:
/// `Assets<Texture>`, `Assets<World>`, `Assets<Shader>`, etc.
#[derive(NotCloneComponent)]
pub struct Assets<T: LoadableAssetTrait> {
    indirection_storage: IndirectionStorage<T>,
    send_drop_channel: SyncGuard<mpsc::Sender<usize>>,
    receive_drop_channel: SyncGuard<mpsc::Receiver<usize>>,
    path_to_handle: HashMap<String, WeakHandle<T>>,
    handle_to_path: HashMap<usize, String>,
    pub asset_loader: T::AssetLoader,
}

unsafe impl<T: LoadableAssetTrait> Send for Assets<T> {}
unsafe impl<T: LoadableAssetTrait> Sync for Assets<T> {}

impl<T: LoadableAssetTrait> Assets<T> {
    pub fn new(default_placeholder: T, asset_loader: T::AssetLoader) -> Self {
        let (send_drop_channel, receive_drop_channel) = mpsc::channel();
        let mut s = Self {
            indirection_storage: IndirectionStorage::new(),
            send_drop_channel: SyncGuard::new(send_drop_channel),
            receive_drop_channel: SyncGuard::new(receive_drop_channel),
            path_to_handle: HashMap::new(),
            handle_to_path: HashMap::new(),
            asset_loader,
        };
        // To ensure the default place-holder stays around forever
        // we drop it without calling its destructor.
        std::mem::forget(s.add(default_placeholder));
        s
    }

    pub fn len(&self) -> usize {
        self.indirection_storage.len()
    }

    pub fn add(&mut self, asset: T) -> Handle<T> {
        let indirection_index = self.indirection_storage.push(asset);
        Handle::new(indirection_index, self.send_drop_channel.inner().clone())
    }

    /// Used to initialize static variables
    /// Adds an asset and leaks it.
    #[allow(dead_code)]
    pub(crate) fn add_and_leak(&mut self, asset: T, handle_to_check: &Handle<T>) {
        let indirection_index = self.indirection_storage.push(asset);
        assert!(indirection_index == handle_to_check.indirection_index);
    }

    pub fn get(&self, handle: &Handle<T>) -> &T {
        self.indirection_storage.get(handle.indirection_index)
    }

    pub fn get_mut(&mut self, handle: &Handle<T>) -> &mut T {
        self.indirection_storage.get_mut(handle.indirection_index)
    }

    pub fn handle_to_path(&self, handle: &Handle<T>) -> Option<&str> {
        self.handle_to_path
            .get(&handle.indirection_index)
            .map(String::as_str)
    }

    pub fn load(&mut self, path: &str) -> Handle<T> {
        self.load_with_options(path, Default::default())
    }

    /// Create a new asset handle initialized to the default placeholder.
    pub fn new_handle(&mut self) -> Handle<T> {
        let indirection_index = self.indirection_storage.get_new_indirection_index();
        let new_handle =
            Handle::<T>::new(indirection_index, self.send_drop_channel.inner().clone());
        new_handle
    }

    pub fn load_with_options(&mut self, path: &str, options: T::Options) -> Handle<T> {
        // Check first if we've already loaded this path.
        // The weak handle upgrade may fail, but if that happens proceed to load a new instance of the asset.
        if let Some(weak_handle) = self.path_to_handle.get(path) {
            println!("AVOIDING EXTRA LOAD");
            if let Some(handle) = weak_handle.upgrade() {
                return handle;
            }
        }

        let new_handle = self.new_handle();
        self.path_to_handle
            .insert(path.to_string(), new_handle.clone_weak());
        self.handle_to_path
            .insert(new_handle.indirection_index, path.to_string());
        self.asset_loader
            .load_with_options(path, new_handle.clone(), options);
        new_handle
    }

    pub fn load_with_data_and_options_and_extension(
        &mut self,
        data: Vec<u8>,
        extension: String,
        options: T::Options,
    ) -> Handle<T> {
        // Could a path be accepted instead of extension to allow for temporary substitutions of assets?
        // Or for gltfs to have subpaths like "some_file.gltf/buffer_data0_100.png"?
        let new_handle = self.new_handle();
        self.asset_loader.load_with_data_and_options_and_extension(
            data,
            extension,
            new_handle.clone(),
            options,
        );
        new_handle
    }

    // Points a `Handle` towards a new asset if it were previously pointed at the default placeholder.
    // Panics if the `Handle` were not previously pointing at a placeholder.
    pub fn replace_placeholder(&mut self, handle: &Handle<T>, asset: T) {
        self.indirection_storage
            .replace_placeholder(handle.indirection_index, asset)
    }

    /// Pass in a closure that will properly clean-up the items that need to be dropped.
    /// This is needed to clean up things like GPU resources.
    pub fn drop_items(&mut self, mut drop_function: impl FnMut(T)) {
        for indirection_index in self.receive_drop_channel.inner().try_iter() {
            let item = self.indirection_storage.remove(indirection_index);
            if let Some(path) = self.handle_to_path.remove(&indirection_index) {
                self.path_to_handle.remove(&path);
            }
            drop_function(item)
        }
    }

    pub fn is_placeholder(&self, handle: &Handle<T>) -> bool {
        self.indirection_storage
            .is_placeholder(handle.indirection_index)
    }
}

pub trait LoadableAssetTrait: Sized + 'static {
    type AssetLoader: AssetLoader<Self> + Send + Sync;
    type Options: Default;
}

pub trait AssetLoader<T: LoadableAssetTrait> {
    fn load_with_options(&mut self, path: &str, handle: Handle<T>, options: T::Options);
    fn load_with_data_and_options_and_extension(
        &mut self,
        _data: Vec<u8>,
        _extension: String,
        _handle: Handle<T>,
        _options: T::Options,
    ) {
        unimplemented!()
    }
}

pub struct SyncGuard<T> {
    inner: T,
}
impl<T> SyncGuard<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
    pub fn inner(&mut self) -> &mut T {
        &mut self.inner
    }
}

/// # Safety
/// Nobody in the Rust Gamedev Discord yelled at me about this.
unsafe impl<T> Sync for SyncGuard<T> {}

#[cfg(not(target_arch = "wasm32"))]
pub async fn fetch_bytes(path: &str) -> Result<Vec<u8>, ()> {
    let contents = std::fs::read(path).unwrap_or_else(|_| panic!("No such path: {:?}", path));
    Ok(contents)
}

#[cfg(target_arch = "wasm32")]
pub async fn fetch_bytes(path: &str) -> Result<Vec<u8>, ()> {
    kwasm::libraries::fetch(path).await
}
