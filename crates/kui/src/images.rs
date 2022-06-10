use crate::texture_atlas::RectU32;
pub struct Images {
    image_data: Vec<ImageDataInternal>,
    free_indices: Vec<usize>,
    send_channel: std::sync::mpsc::Sender<usize>,
    receive_channel: std::sync::mpsc::Receiver<usize>,
    packer: rect_packer::Packer,
    newly_packed: Vec<usize>,
}

struct ImageDataInternal {
    image_data: ImageData,
    location: Option<RectU32>,
}
pub struct ImageData {
    pub width: usize,
    pub height: usize,
    /// RGBA8 data
    pub data: Vec<u8>,
}

struct ImageDropHandle {
    index: usize,
    drop_channel: std::sync::mpsc::Sender<usize>,
}

impl Drop for ImageDropHandle {
    fn drop(&mut self) {
        let _ = self.drop_channel.send(self.index);
    }
}

#[derive(Clone)]
pub struct ImageHandle(std::sync::Arc<ImageDropHandle>);

impl Images {
    fn get_packer() -> rect_packer::Packer {
        let initial_size = 1024;

        let rect_packer_config = rect_packer::Config {
            width: initial_size as i32,
            height: initial_size as i32,
            border_padding: 5,
            rectangle_padding: 10,
        };
        rect_packer::Packer::new(rect_packer_config)
    }
    pub fn new() -> Self {
        let (send_channel, receive_channel) = std::sync::mpsc::channel();

        Self {
            image_data: Vec::new(),
            free_indices: Vec::new(),
            send_channel,
            receive_channel,
            packer: Self::get_packer(),
            newly_packed: Vec::new(),
        }
    }

    fn unload_dropped(&mut self) {
        for drop in self.receive_channel.iter() {
            let dropped_image = &mut self.image_data[drop];
            dropped_image.image_data.data.clear();
            dropped_image.image_data.width = 0;
            dropped_image.image_data.height = 0;
            dropped_image.location = None;
            self.free_indices.push(drop);
        }
    }

    pub fn add_image(&mut self, image_data: ImageData) -> ImageHandle {
        self.unload_dropped();

        let index = if let Some(free_index) = self.free_indices.pop() {
            self.image_data[free_index].image_data = image_data;
            free_index
        } else {
            self.image_data.push(ImageDataInternal {
                location: None,
                image_data,
            });
            self.image_data.len() - 1
        };

        ImageHandle(std::sync::Arc::new(ImageDropHandle {
            index,
            drop_channel: self.send_channel.clone(),
        }))
    }

    fn repack(&mut self) {
        self.packer = Self::get_packer();
        for image in self.image_data.iter_mut() {
            image.location = None;
        }

        let mut packed_in_last_frame = Vec::new();
        std::mem::swap(&mut packed_in_last_frame, &mut self.newly_packed);
        for index in packed_in_last_frame {
            self.pack_inner(index, false);
        }
    }

    fn pack_inner(&mut self, image_index: usize, allow_repack: bool) -> Option<RectU32> {
        if let Some(location) = self.image_data[image_index].location {
            Some(location)
        } else {
            let Self {
                image_data, packer, ..
            } = self;
            let image_data = &mut image_data[image_index];

            let rect = packer.pack(
                image_data.image_data.width as i32,
                image_data.image_data.width as i32,
                false,
            );
            if let Some(rect) = rect {
                self.newly_packed.push(image_index);
                Some(RectU32::new(
                    rect.x as u32,
                    rect.y as u32,
                    rect.width as u32,
                    rect.height as u32,
                ))
            } else if allow_repack {
                // If we can't fit this repack and remove everything that hasn't been rendered this frame.
                self.repack();
                self.pack_inner(image_index, false)
            } else {
                None
            }
        }
    }

    pub fn get_image_texture_rect(&mut self, image_handle: &ImageHandle) -> Option<RectU32> {
        self.unload_dropped();
        self.pack_inner(image_handle.0.index, true)
    }

    /// Call this once per frame to get the subrects and data that need to be updated in the GPU texture.
    pub fn update_rects(&mut self, update_rects: impl Fn(RectU32, &[u8])) {
        for i in self.newly_packed.drain(..) {
            let image_data = &self.image_data[i];
            if let Some(rect) = image_data.location {
                update_rects(rect, &image_data.image_data.data)
            }
        }
    }
}
