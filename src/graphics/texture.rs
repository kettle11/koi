use crate::*;
use kgraphics::*;

pub use kgraphics::Texture;

use std::sync::mpsc;

struct TextureLoadMessage {
    handle: Handle<Texture>,
    texture_load_data: TextureLoadData,
    texture_settings: TextureSettings,
}

pub struct TextureLoadData {
    pub data: Vec<u8>,
    pub pixel_format: PixelFormat,
    pub width: u32,
    pub height: u32,
}

pub struct TextureAssetLoader {
    sender: SyncGuard<mpsc::Sender<TextureLoadMessage>>,
    receiver: SyncGuard<mpsc::Receiver<TextureLoadMessage>>,
}

/// A system that loads textures onto the GPU
pub(crate) fn load_textures(textures: &mut Assets<Texture>, graphics: &mut Graphics) {
    // A Vec doesn't need to be allocated here.
    // This is just a way to not borrow the TextureAssetLoader and Textures at
    // the same time.
    let messages: Vec<TextureLoadMessage> =
        textures.asset_loader.receiver.inner().try_iter().collect();
    for message in messages.into_iter() {
        let texture = graphics
            .new_texture(
                Some(&message.texture_load_data.data),
                message.texture_load_data.width,
                message.texture_load_data.height,
                message.texture_load_data.pixel_format,
                message.texture_settings,
            )
            .unwrap();

        textures.replace_placeholder(&message.handle, texture);
    }
}

#[cfg(feature = "png")]
pub fn new_texture_from_png_bytes(
    graphics: &mut Graphics,
    texture_settings: TextureSettings,
    bytes: &[u8],
) -> Texture {
    let texture_load_data = png_data_from_bytes(bytes);
    graphics
        .new_texture(
            Some(&texture_load_data.data),
            texture_load_data.width,
            texture_load_data.height,
            texture_load_data.pixel_format,
            texture_settings,
        )
        .unwrap()
}

#[cfg(feature = "jpeg")]
pub fn new_texture_from_jpeg_bytes(
    graphics: &mut Graphics,
    texture_settings: TextureSettings,
    bytes: &[u8],
) -> Texture {
    let texture_load_data = jpeg_data_from_bytes(bytes);
    graphics
        .new_texture(
            Some(&texture_load_data.data),
            texture_load_data.width,
            texture_load_data.height,
            texture_load_data.pixel_format,
            texture_settings,
        )
        .unwrap()
}

#[cfg(feature = "png")]
pub fn png_data_from_bytes(bytes: &[u8]) -> TextureLoadData {
    let reader = std::io::BufReader::new(&bytes as &[u8]);
    let mut decoder = png::Decoder::new(reader);

    // This line reduces 16-bit or greater images to 8 bit.
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().unwrap();
    let mut pixels = vec![0; reader.output_buffer_size()];
    let metadata = reader.next_frame(&mut pixels).unwrap();

    let pixel_format = match metadata.color_type {
        // png::ColorType::Rgb => PixelFormat::RGB8Unorm,
        png::ColorType::Rgba => PixelFormat::RGBA8Unorm,
        png::ColorType::Grayscale => PixelFormat::R8Unorm,
        png::ColorType::Rgb => PixelFormat::RGB8Unorm,
        //  png::ColorType::GrayscaleAlpha => PixelFormat::RG8Unorm, // Is this correct?
        _ => unimplemented!("Unsupported PNG pixel format: {:?}", metadata.color_type,),
    };
    TextureLoadData {
        data: pixels,
        pixel_format,
        width: metadata.width,
        height: metadata.height,
    }
}

#[cfg(feature = "jpeg")]
pub fn jpeg_data_from_bytes(bytes: &[u8]) -> TextureLoadData {
    let reader = std::io::BufReader::new(&bytes as &[u8]);

    let mut decoder = jpeg_decoder::Decoder::new(reader);
    let pixels = decoder.decode().expect("failed to decode image");
    let metadata = decoder.info().unwrap();

    let pixel_format = match metadata.pixel_format {
        jpeg_decoder::PixelFormat::RGB24 => PixelFormat::RGB8Unorm,
        jpeg_decoder::PixelFormat::L8 => PixelFormat::R8Unorm,
        jpeg_decoder::PixelFormat::CMYK32 => {
            panic!("CMYK is currently unsupported")
        } // _ => unimplemented!("Unsupported Jpeg pixel format: {:?}", metadata.pixel_format,),
    };
    TextureLoadData {
        data: pixels,
        pixel_format,
        width: metadata.width as u32,
        height: metadata.height as u32,
    }
}

impl AssetLoader<Texture> for TextureAssetLoader {
    fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            sender: SyncGuard::new(sender),
            receiver: SyncGuard::new(receiver),
        }
    }

    fn load_with_options(
        &mut self,
        path: &str,
        handle: Handle<Texture>,
        options: <Texture as LoadableAssetTrait>::Options,
    ) {
        let path = path.to_owned();
        let sender = self.sender.inner().clone();

        ktasks::spawn(async move {
            let extension = std::path::Path::new(&path)
                .extension()
                .and_then(std::ffi::OsStr::to_str);

            let result = match extension {
                #[cfg(feature = "png")]
                Some("png") => {
                    let bytes = crate::fetch_bytes(&path)
                        .await
                        .expect(&format!("Failed to open file: {}", path));
                    let texture_load_data = png_data_from_bytes(&bytes);

                    TextureLoadMessage {
                        texture_load_data,
                        handle,
                        texture_settings: options,
                    }
                }
                #[cfg(feature = "jpeg")]
                Some("jpg") | Some("jpeg") => {
                    let bytes = crate::fetch_bytes(&path)
                        .await
                        .expect(&format!("Failed to open file: {}", path));
                    let texture_load_data = jpeg_data_from_bytes(&bytes);
                    TextureLoadMessage {
                        texture_load_data,
                        handle,
                        texture_settings: options,
                    }
                }
                None => panic!("No file extension for path: {:?}", path),
                _ => panic!("Unsupported texture extension: {:?}", path),
            };
            // Send data to GPU AssetLoader channel.
            // Potentially this could occur if somehow the main thread shuts down first.
            // But in that case it's OK to simply do nothing.
            let _ = sender.send(result);
        })
        .run();
    }
}
impl LoadableAssetTrait for Texture {
    type Options = TextureSettings;
    type AssetLoader = TextureAssetLoader;
}

pub const WHITE_TEXTURE: Handle<Texture> = Handle::<Texture>::new_with_just_index(1);
pub const BLACK_TEXTURE: Handle<Texture> = Handle::<Texture>::new_with_just_index(2);

/// A texture that produces normals that all face outwards.
/// The color is (0.5, 0.5, 1.0)
pub const NORMAL_TEXTURE: Handle<Texture> = Handle::<Texture>::new_with_just_index(3);

pub fn initialize_static_textures(graphics: &mut Graphics, textures: &mut Assets<Texture>) {
    textures.add_and_leak(
        graphics
            .new_texture(
                Some(&[255, 255, 255, 255]),
                1,
                1,
                PixelFormat::RGBA8Unorm,
                TextureSettings {
                    srgb: false,
                    ..Default::default()
                },
            )
            .unwrap(),
        &WHITE_TEXTURE,
    );
    textures.add_and_leak(
        graphics
            .new_texture(
                Some(&[0, 0, 0, 255]),
                1,
                1,
                PixelFormat::RGBA8Unorm,
                TextureSettings {
                    srgb: false,
                    ..Default::default()
                },
            )
            .unwrap(),
        &BLACK_TEXTURE,
    );
    textures.add_and_leak(
        graphics
            .new_texture(
                Some(&[128, 128, 255, 255]),
                1,
                1,
                PixelFormat::RGBA8Unorm,
                TextureSettings {
                    srgb: false,
                    ..Default::default()
                },
            )
            .unwrap(),
        &NORMAL_TEXTURE,
    );
}
