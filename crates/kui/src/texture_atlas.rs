use std::collections::{HashMap, HashSet};

pub struct TextureAtlas {
    pub data: Vec<u8>,
    pub data_swap: Vec<u8>,
    pub width: u32,
    pub height: u32,
    packer: rect_packer::Packer,
    characters: HashMap<fontdue::layout::GlyphRasterConfig, RectU32>,
    last_frame_characters: HashSet<fontdue::layout::GlyphRasterConfig>,
    this_frame_characters: HashSet<fontdue::layout::GlyphRasterConfig>,
    already_repacked: bool,
    pub changed: bool,
}

impl TextureAtlas {
    /// Size should be power of 2
    pub fn new(size: u32) -> Self {
        let config = rect_packer::Config {
            width: size as i32,
            height: size as i32,
            border_padding: 5,
            rectangle_padding: 10,
        };

        Self {
            data: vec![0; (size * size) as usize],
            data_swap: vec![0; (size * size) as usize],
            width: size,
            height: size,
            packer: rect_packer::Packer::new(config),
            characters: HashMap::new(),
            last_frame_characters: HashSet::new(),
            this_frame_characters: HashSet::new(),
            already_repacked: false,
            changed: false,
        }
    }

    /*
    pub fn new_frame(&mut self) {
        std::mem::swap(
            &mut self.this_frame_characters,
            &mut self.last_frame_characters,
        );
        self.already_repacked = false;
        self.this_frame_characters.clear();
    }

    pub fn get_character_no_rasterize(
        &self,
        c: fontdue::layout::GlyphRasterConfig,
    ) -> Option<RectU32> {
        self.characters.get(&c).map_or(None, |i| Some(*i))
    }
    */

    pub fn get_character(
        &mut self,
        font: &fontdue::Font,
        c: fontdue::layout::GlyphRasterConfig,
    ) -> Option<RectU32> {
        if let Some(rectangle) = self.characters.get(&c) {
            self.this_frame_characters.insert(c);
            Some(*rectangle)
        } else {
            let (metrics, new_data) = font.rasterize_config(c);

            if metrics.width == 0 && metrics.height == 0 {
                return Some(RectU32::new(0, 0, 0, 0));
            }
            let rectangle = self.pack_character(c, metrics.width as u32, metrics.height as u32);

            if let Some(rectangle) = rectangle {
                self.this_frame_characters.insert(c);
                self.characters.insert(c, rectangle);

                let mut new_data_index = 0;
                for j in rectangle.y..(rectangle.y + rectangle.height) {
                    for i in rectangle.x..(rectangle.x + rectangle.width) {
                        self.data[(j * self.width + i) as usize] = new_data[new_data_index];
                        new_data_index += 1;
                    }
                }

                // use std::io::Write;
                // let mut o = std::fs::File::create(format!("atlas.pgm",)).unwrap();
                // let _ = o.write(format!("P5\n{} {}\n255\n", self.width, self.height).as_bytes());
                // let _ = o.write(&self.data);
                self.changed = true;
                Some(rectangle)
            } else {
                None
            }
        }
    }

    fn pack_character(
        &mut self,
        c: fontdue::layout::GlyphRasterConfig,
        width: u32,
        height: u32,
    ) -> Option<RectU32> {
        // Just crash for now if there's not space for character.
        let rect = self.packer.pack(width as i32, height as i32, false);
        if let Some(rect) = rect {
            let rectangle = RectU32::new(
                rect.x as u32,
                rect.y as u32,
                rect.width as u32,
                rect.height as u32,
            );

            Some(rectangle)
        } else {
            // We failed to pack the character, try to repack characters using only the characters from the last frame
            // and this frame.
            // If a repack has already occurred this frame do not perform another one.
            if !self.already_repacked {
                self.packer = rect_packer::Packer::new(rect_packer::Config {
                    width: self.width as i32,
                    height: self.height as i32,
                    border_padding: 5,
                    rectangle_padding: 10,
                });

                // Clear all data in the texture
                for i in self.data_swap.iter_mut() {
                    *i = 0;
                }

                self.already_repacked = true;
                let mut new_characters = HashMap::new();

                // Repack characters already used this frame
                Self::copy_characters(
                    &mut self.packer,
                    self.width,
                    &self.data,
                    &mut self.data_swap,
                    &self.this_frame_characters,
                    &self.characters,
                    &mut new_characters,
                );

                // Try again after we've repacked everything else from this frame.
                let packed = self.pack_character(c, width, height);

                // Pack characters from the last frame
                Self::copy_characters(
                    &mut self.packer,
                    self.width,
                    &self.data,
                    &mut self.data_swap,
                    &self.last_frame_characters,
                    &self.characters,
                    &mut new_characters,
                );

                std::mem::swap(&mut self.data, &mut self.data_swap);
                self.characters = new_characters;
                self.last_frame_characters.clear();
                self.this_frame_characters.clear();

                println!("SAVING");
                use std::io::Write;
                let mut o = std::fs::File::create("atlas.pgm".to_string()).unwrap();
                let _ = o.write(format!("P5\n{} {}\n255\n", self.width, self.height).as_bytes());
                let _ = o.write(&self.data);

                packed
            } else {
                None
            }
        }
    }

    /// Copy characters to new texture data buffer
    fn copy_characters(
        packer: &mut rect_packer::Packer,
        texture_width: u32,
        old_data: &[u8],
        new_data: &mut Vec<u8>,
        copying_characters: &HashSet<fontdue::layout::GlyphRasterConfig>,
        characters: &HashMap<fontdue::layout::GlyphRasterConfig, RectU32>,
        new_characters: &mut HashMap<fontdue::layout::GlyphRasterConfig, RectU32>,
    ) {
        for c in copying_characters.iter() {
            let old_rectangle = characters[c];
            let new_rectangle = packer.pack(
                old_rectangle.width as i32,
                old_rectangle.height as i32,
                false,
            );

            if let Some(new_rectangle) = new_rectangle {
                let new_x = new_rectangle.x as u32;
                let new_y = new_rectangle.y as u32;
                let width = old_rectangle.width;
                let height = old_rectangle.height;
                for j in 0..height {
                    for i in 0..width {
                        new_data[((j + new_y) * texture_width + i + new_x) as usize] =
                            old_data[((j + old_rectangle.y) * texture_width + i + old_rectangle.x)
                                as usize];
                    }
                }

                let rectangle = RectU32::new(
                    new_rectangle.x as u32,
                    new_rectangle.y as u32,
                    new_rectangle.width as u32,
                    new_rectangle.height as u32,
                );

                // Add character to new hashmap.
                new_characters.insert(*c, rectangle);
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct RectU32 {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl RectU32 {
    fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}
