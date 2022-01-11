use crate::*;
use core::ops::Deref;
use kgraphics::*;

use std::sync::mpsc;

pub struct Texture(pub(crate) kgraphics::Texture);

impl Deref for Texture {
    type Target = kgraphics::Texture;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
    let texture_load_data = png_data_from_bytes(bytes, texture_settings.srgb);
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
    let texture_load_data = jpeg_data_from_bytes(bytes, texture_settings.srgb);
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

fn extend_pixels_1_with_alpha(pixels: Vec<u8>) -> Vec<u8> {
    pixels.iter().flat_map(|a| [*a, *a, *a, 255]).collect()
}

fn extend_pixels_3_with_alpha(pixels: Vec<u8>) -> Vec<u8> {
    pixels
        .chunks_exact(3)
        .flat_map(|a| [a[0], a[1], a[2], 255])
        .collect()
}

#[cfg(feature = "png")]
pub fn png_data_from_bytes(bytes: &[u8], srgb: bool) -> TextureLoadData {
    let reader = std::io::BufReader::new(bytes);
    let mut decoder = png::Decoder::new(reader);

    // This line reduces 16-bit or greater images to 8 bit.
    decoder.set_transformations(png::Transformations::normalize_to_color8());
    let mut reader = decoder.read_info().unwrap();
    let mut pixels = vec![0; reader.output_buffer_size()];
    let metadata = reader.next_frame(&mut pixels).unwrap();

    let pixel_format = match metadata.color_type {
        // png::ColorType::Rgb => PixelFormat::RGB8Unorm,
        png::ColorType::Rgba => PixelFormat::RGBA8Unorm,
        png::ColorType::Grayscale => {
            // Convert to RGBA sRGB8_ALPHA8 is the only color renderable format
            // which is allowed for mipmap generation
            if srgb {
                pixels = extend_pixels_1_with_alpha(pixels);
                PixelFormat::RGBA8Unorm
            } else {
                PixelFormat::R8Unorm
            }
        }
        png::ColorType::Rgb => {
            // Convert to RGBA sRGB8_ALPHA8 is the only color renderable format
            // which is required for mipmap generation
            if srgb {
                pixels = extend_pixels_3_with_alpha(pixels);
                PixelFormat::RGBA8Unorm
            } else {
                PixelFormat::RGB8Unorm
            }
        }
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

#[cfg(feature = "imagine_png")]
pub fn png_data_from_bytes(bytes: &[u8], _srgb: bool) -> TextureLoadData {
    let (data, width, height) = imagine_integration::parse_me_a_png_yo(bytes).unwrap();

    let mut final_data = Vec::with_capacity(data.len() * 4);
    for imagine::RGBA8 { r, g, b, a } in data {
        final_data.push(r);
        final_data.push(g);
        final_data.push(b);
        final_data.push(a);
    }
    TextureLoadData {
        data: final_data,
        pixel_format: PixelFormat::RGBA8Unorm,
        width,
        height,
    }
}

#[cfg(feature = "jpeg")]
pub fn jpeg_data_from_bytes(bytes: &[u8], srgb: bool) -> TextureLoadData {
    let reader = std::io::BufReader::new(bytes);

    let mut decoder = jpeg_decoder::Decoder::new(reader);
    let mut pixels = decoder.decode().expect("failed to decode image");
    let metadata = decoder.info().unwrap();

    let pixel_format = match metadata.pixel_format {
        jpeg_decoder::PixelFormat::RGB24 => {
            // Convert to RGBA sRGB8_ALPHA8 is the only color renderable format
            // which is required for mipmap generation
            if srgb {
                pixels = extend_pixels_3_with_alpha(pixels);
                PixelFormat::RGBA8Unorm
            } else {
                PixelFormat::RGB8Unorm
            }
        }
        jpeg_decoder::PixelFormat::L8 => {
            // Convert to RGBA sRGB8_ALPHA8 is the only color renderable format
            // which is required for mipmap generation
            if srgb {
                pixels = extend_pixels_1_with_alpha(pixels);
                PixelFormat::RGBA8Unorm
            } else {
                PixelFormat::R8Unorm
            }
        }
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

#[cfg(feature = "hdri")]
pub fn hdri_data_from_bytes(bytes: &[u8]) -> TextureLoadData {
    // This data is always assumed to be linear sRGB
    let image = hdrldr::load(bytes).expect("Failed to decode HDRI image data");

    // Pad with alpha.
    // Some platforms (Firefox on web) don't support RGB32F well.
    let mut texture: Vec<[f32; 4]> = Vec::with_capacity(image.data.len());
    for hdrldr::RGB { r, g, b } in image.data {
        texture.push([r, g, b, 0.0]);
    }

    unsafe {
        // Is this conversion correct? I believe so because [f32; 4] is aligned on u8 boundaries.
        let data: Vec<u8> = Vec::from_raw_parts(
            texture.as_mut_ptr() as *mut u8,
            texture.len() * 4 * std::mem::size_of::<f32>(),
            texture.capacity() * 4 * std::mem::size_of::<f32>(),
        );
        texture.leak();

        TextureLoadData {
            data,
            width: image.width as u32,
            height: image.height as u32,
            pixel_format: PixelFormat::RGBA32F,
        }
    }
}

pub(crate) async fn texture_data_from_path(
    path: &str,
    options: &mut TextureSettings,
) -> TextureLoadData {
    let extension = std::path::Path::new(&path)
        .extension()
        .and_then(std::ffi::OsStr::to_str);

    let bytes = crate::fetch_bytes(&path)
        .await
        .unwrap_or_else(|_| panic!("Failed to open file: {}", path));
    match extension {
        #[cfg(any(feature = "png", feature = "imagine_png"))]
        Some("png") => png_data_from_bytes(&bytes, options.srgb),
        #[cfg(feature = "jpeg")]
        Some("jpg") | Some("jpeg") => jpeg_data_from_bytes(&bytes, options.srgb),
        #[cfg(feature = "hdri")]
        Some("hdr") => {
            options.srgb = false;
            hdri_data_from_bytes(&bytes)
        }
        None => panic!("No file extension for path: {:?}", path),
        _ => panic!("Unsupported texture extension: {:?}", path),
    }
}

impl TextureAssetLoader {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            sender: SyncGuard::new(sender),
            receiver: SyncGuard::new(receiver),
        }
    }
}

impl AssetLoader<Texture> for TextureAssetLoader {
    fn load_with_options(
        &mut self,
        path: &str,
        handle: Handle<Texture>,
        mut options: <Texture as LoadableAssetTrait>::Options,
    ) {
        let path = path.to_owned();
        let sender = self.sender.inner().clone();

        ktasks::spawn(async move {
            let texture_load_data = texture_data_from_path(&path, &mut options).await;

            // Send data to GPU AssetLoader channel.
            // Potentially this could occur if somehow the main thread shuts down first.
            // But in that case it's OK to simply do nothing.
            let _ = sender.send(TextureLoadMessage {
                texture_load_data,
                handle,
                texture_settings: options,
            });
        })
        .run();
    }
}
impl LoadableAssetTrait for Texture {
    type Options = TextureSettings;
    type AssetLoader = TextureAssetLoader;
}

impl Texture {
    pub const WHITE: Handle<Texture> = Handle::<Texture>::new_with_just_index(1);
    pub const BLACK: Handle<Texture> = Handle::<Texture>::new_with_just_index(2);

    /// A texture that produces normals that all face outwards.
    /// The color is (0.5, 0.5, 1.0)
    pub const NORMAL: Handle<Texture> = Handle::<Texture>::new_with_just_index(3);
    pub const BLUE: Handle<Texture> = Handle::<Texture>::new_with_just_index(4);
}

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
        &Texture::WHITE,
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
        &Texture::BLACK,
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
        &Texture::NORMAL,
    );
    textures.add_and_leak(
        graphics
            .new_texture(
                Some(&[0, 0, 255, 255]),
                1,
                1,
                PixelFormat::RGBA8Unorm,
                TextureSettings {
                    srgb: false,
                    ..Default::default()
                },
            )
            .unwrap(),
        &Texture::BLUE,
    );
}
