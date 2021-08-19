/// An unsafe bump allocator used to store uniforms.
pub struct BumpAllocator {
    pub(crate) data: Vec<u8>,
}

/*
pub struct BumpHandle<T: Sized> {
    offset: usize,
    size: usize,
    phantom: std::marker::PhantomData<T>,
}*/

#[derive(Clone)]
pub struct BumpHandle {
    offset: usize,
    _size: usize,
}

impl BumpAllocator {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.data.clear()
    }

    pub fn push<T: Sized + 'static>(&mut self, item: T) -> BumpHandle {
        unsafe {
            let offset = self.data.len();
            let size = std::mem::size_of::<T>();

            let bytes = std::slice::from_raw_parts((&item as *const T) as *const u8, size);

            self.data.extend(bytes);

            // Align to byte boundary
            let byte_align_remainder = (size * offset) % 4;
            self.data.reserve(byte_align_remainder);
            for _ in 0..byte_align_remainder {
                self.data.push(0)
            }

            BumpHandle {
                offset,
                _size: size,
            }
        }
    }

    /// This is super unsafe if it's the incorrect type
    pub unsafe fn get_any<T: Sized>(&self, handle: BumpHandle) -> &T {
        &*((&self.data[handle.offset]) as *const _ as *const T)
    }
}
