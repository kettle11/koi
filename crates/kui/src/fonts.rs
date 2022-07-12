#[derive(Copy, Clone, Debug, Default)]
pub struct Font(pub(crate) usize);

impl Font {
    pub const fn from_index(index: usize) -> Self {
        Font(index)
    }
}

pub trait GetFonts {
    fn get_fonts(&self) -> &Fonts;
    fn get_fonts_mut(&mut self) -> &Fonts;
}

pub struct Fonts {
    fonts: Vec<fontdue::Font>,
}

impl Default for Fonts {
    fn default() -> Self {
        let mut s = Self::empty();
        s.load_default_fonts();
        s
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use fontdb::Family;

impl Fonts {
    pub fn empty() -> Self {
        Self { fonts: Vec::new() }
    }

    pub fn load_default_fonts(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut db = fontdb::Database::new();
            db.load_system_fonts();

            /*
            for face in db.faces() {
                println!("FONT FACE: {:?}", face.family)
            }
            */

            let query = fontdb::Query {
                // Default to Helvetica, otherwise fallback on an available system font.
                // This should be changed later.
                families: &[fontdb::Family::Name("Helvetica"), fontdb::Family::SansSerif],
                weight: fontdb::Weight::NORMAL,
                style: fontdb::Style::Normal,
                stretch: fontdb::Stretch::Normal,
            };

            match db.query(&query) {
                Some(id) => {
                    let (src, _) = db.face_source(id).unwrap();
                    if let fontdb::Source::File(ref path) = &src {
                        println!("Selected font: {:?}", path);
                        let bytes = std::fs::read(path)
                            .unwrap_or_else(|_| panic!("No such path: {:?}", path));
                        self.new_font_from_bytes(&bytes).unwrap();
                    }
                }
                None => {
                    println!("Could not find default system font");
                }
            }
        }

        #[cfg(all(target_arch = "wasm32", feature = "default_font"))]
        {
            self.new_font_from_bytes(include_bytes!("../Inter-Regular.otf"))
                .unwrap();
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn load_system_font(&mut self, families: &[fontdb::Family]) -> Result<Font, &'static str> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mut db = fontdb::Database::new();
            db.load_system_fonts();

            let query = fontdb::Query {
                // Default to Helvetica, otherwise fallback on an available system font.
                // This should be changed later.
                families,
                weight: fontdb::Weight::NORMAL,
                style: fontdb::Style::Normal,
                stretch: fontdb::Stretch::Normal,
            };

            match db.query(&query) {
                Some(id) => {
                    let (src, _) = db.face_source(id).unwrap();
                    if let fontdb::Source::File(ref path) = &src {
                        println!("Selected font: {:?}", path);
                        let bytes = std::fs::read(path)
                            .unwrap_or_else(|_| panic!("No such path: {:?}", path));
                        Ok(self.new_font_from_bytes(&bytes).unwrap())
                    } else {
                        Result::Err("Could not find system font")
                    }
                }
                None => Result::Err("Could not find default system font"),
            }
        }
    }

    pub fn new_font_from_bytes(&mut self, data: &[u8]) -> Result<Font, &'static str> {
        self.fonts.push(fontdue::Font::from_bytes(
            data,
            fontdue::FontSettings::default(),
        )?);
        Ok(Font(self.fonts.len() - 1))
    }

    pub fn fonts(&self) -> &[fontdue::Font] {
        &self.fonts
    }
}
