use imagine::png::*;
use imagine::*;
use std::convert::TryFrom;

pub(crate) fn parse_me_a_png_yo(
    png: &[u8],
) -> Result<(Vec<imagine::RGBA8>, u32, u32), imagine::png::PngError> {
    let mut it = RawPngChunkIter::new(png)
        .map(PngChunk::try_from)
        .filter(critical_errors_only);
    let ihdr = it
        .next()
        .ok_or(PngError::NoChunksPresent)??
        .to_ihdr()
        .ok_or(PngError::FirstChunkNotIHDR)?;

    let mut palette: Option<&[RGB8]> = None;
    let mut transparency: Option<tRNS> = None;

    let idat_peek = it.peekable();
    let idat_slice_it = idat_peek.filter_map(|r_chunk| match r_chunk {
        Ok(PngChunk::IDAT(IDAT { data })) => Some(data),
        Ok(PngChunk::PLTE(PLTE { data })) => {
            palette = Some(data);
            None
        }
        Ok(PngChunk::tRNS(t)) => {
            transparency = Some(t);
            None
        }
        // TODO: support utilizing background chunks.
        Ok(PngChunk::iCCP(_)) => None,
        Ok(_other) => None,
        _ => None,
    });
    let mut temp_memory_buffer = vec![0; ihdr.temp_memory_requirement()];
    decompress_idat_to_temp_storage(&mut temp_memory_buffer, idat_slice_it)?;
    //
    let mut final_storage = Vec::new();
    final_storage.resize(
        (ihdr.width.saturating_mul(ihdr.height)) as usize,
        RGBA8::default(),
    );
    //
    match ihdr.pixel_format {
        // we already have all four channels
        PngPixelFormat::RGBA8 => {
            unfilter_decompressed_data(ihdr, &mut temp_memory_buffer, |x, y, data| {
                let rgba8: RGBA8 = bytemuck::cast_slice(data)[0];
                final_storage[(y * ihdr.width + x) as usize] = rgba8;
            })?
        }
        PngPixelFormat::RGBA16 => {
            // TODO: some day we might want to display the full 16-bit channels, WGPU
            // supports it, we think.
            unfilter_decompressed_data(ihdr, &mut temp_memory_buffer, |x, y, data| {
                let rgba16: [u16; 4] = [
                    u16::from_be_bytes([data[0], data[1]]),
                    u16::from_be_bytes([data[2], data[3]]),
                    u16::from_be_bytes([data[4], data[5]]),
                    u16::from_be_bytes([data[6], data[7]]),
                ];
                let rgba8 = rgba16_to_rgba8(rgba16);
                final_storage[(y * ihdr.width + x) as usize] = rgba8;
            })?
        }

        // with rgb only, it adds alpha as fully opaque
        PngPixelFormat::RGB8 => {
            unfilter_decompressed_data(ihdr, &mut temp_memory_buffer, |x, y, data| {
                let rgb8: RGB8 = bytemuck::cast_slice(data)[0];
                let mut rgba8 = rgb8_to_rgba8(rgb8);
                if let Some(rgb8_trns_key) = transparency.and_then(tRNS::to_rgb8) {
                    if rgb8 == rgb8_trns_key {
                        rgba8.a = 0;
                    }
                };
                final_storage[(y * ihdr.width + x) as usize] = rgba8;
            })?
        }
        PngPixelFormat::RGB16 => {
            unfilter_decompressed_data(ihdr, &mut temp_memory_buffer, |x, y, data| {
                let rgb16: [u16; 3] = [
                    u16::from_be_bytes([data[0], data[1]]),
                    u16::from_be_bytes([data[2], data[3]]),
                    u16::from_be_bytes([data[4], data[5]]),
                ];
                let mut rgba8 = rgb16_to_rgba8(rgb16);
                if let Some(rgb16_trns_key) = transparency.and_then(tRNS::rgb) {
                    if rgb16 == rgb16_trns_key {
                        rgba8.a = 0;
                    }
                };
                final_storage[(y * ihdr.width + x) as usize] = rgba8;
            })?
        }

        // grayscale
        PngPixelFormat::Y1 => {
            unfilter_decompressed_data(ihdr, &mut temp_memory_buffer, |x, y, data| {
                let y1 = bytemuck::cast_slice(data)[0];
                let mut rgba8 = y1_to_rgba8(y1);
                if let Some(y8_trns_key) = transparency.and_then(tRNS::to_y8) {
                    if y1 == y8_trns_key.y {
                        rgba8.a = 0;
                    }
                };
                final_storage[(y * ihdr.width + x) as usize] = rgba8;
            })?
        }
        PngPixelFormat::Y2 => {
            unfilter_decompressed_data(ihdr, &mut temp_memory_buffer, |x, y, data| {
                let y2 = bytemuck::cast_slice(data)[0];
                let mut rgba8 = y2_to_rgba8(y2);
                if let Some(y8_trns_key) = transparency.and_then(tRNS::to_y8) {
                    if y2 == y8_trns_key.y {
                        rgba8.a = 0;
                    }
                };
                final_storage[(y * ihdr.width + x) as usize] = rgba8;
            })?
        }
        PngPixelFormat::Y4 => {
            unfilter_decompressed_data(ihdr, &mut temp_memory_buffer, |x, y, data| {
                let y4 = bytemuck::cast_slice(data)[0];
                let mut rgba8 = y4_to_rgba8(y4);
                if let Some(y8_trns_key) = transparency.and_then(tRNS::to_y8) {
                    if y4 == y8_trns_key.y {
                        rgba8.a = 0;
                    }
                };
                final_storage[(y * ihdr.width + x) as usize] = rgba8;
            })?
        }
        PngPixelFormat::Y8 => {
            unfilter_decompressed_data(ihdr, &mut temp_memory_buffer, |x, y, data| {
                let y8 = bytemuck::cast_slice(data)[0];
                let mut rgba8 = y8_to_rgba8(y8);
                if let Some(y8_trns_key) = transparency.and_then(tRNS::to_y8) {
                    if y8 == y8_trns_key.y {
                        rgba8.a = 0;
                    }
                };
                final_storage[(y * ihdr.width + x) as usize] = rgba8;
            })?
        }
        PngPixelFormat::Y16 => {
            unfilter_decompressed_data(ihdr, &mut temp_memory_buffer, |x, y, data| {
                let y16: u16 = u16::from_be_bytes(data.try_into().unwrap());
                let mut rgba8 = y8_to_rgba8((y16 >> 8) as u8);
                if let Some(y16_trns_key) = transparency.and_then(tRNS::y) {
                    if y16 == y16_trns_key {
                        rgba8.a = 0;
                    }
                };
                final_storage[(y * ihdr.width + x) as usize] = rgba8;
            })?
        }

        // also grayscale, but now we already have an alpha value we keep
        PngPixelFormat::YA8 => {
            unfilter_decompressed_data(ihdr, &mut temp_memory_buffer, |x, y, data| {
                let ya8: YA8 = bytemuck::cast_slice(data)[0];
                let mut rgba8 = y8_to_rgba8(ya8.y);
                rgba8.a = ya8.a;
                final_storage[(y * ihdr.width + x) as usize] = rgba8;
            })?
        }
        PngPixelFormat::YA16 => {
            unfilter_decompressed_data(ihdr, &mut temp_memory_buffer, |x, y, data| {
                let ya16: [u16; 2] = [
                    u16::from_be_bytes([data[0], data[1]]),
                    u16::from_be_bytes([data[2], data[3]]),
                ];
                let mut rgba8 = y8_to_rgba8((ya16[0] >> 8) as u8);
                rgba8.a = (ya16[1] >> 8) as u8;
                final_storage[(y * ihdr.width + x) as usize] = rgba8;
            })?
        }

        // indexed color looks into the palette (or black)
        PngPixelFormat::I1 | PngPixelFormat::I2 | PngPixelFormat::I4 | PngPixelFormat::I8 => {
            unfilter_decompressed_data(ihdr, &mut temp_memory_buffer, |x, y, data| {
                let index = data[0] as usize;
                let rgb8 = palette
                    .map(|pal| match pal.get(index) {
                        Some(thing) => *thing,
                        None => RGB8::default(),
                    })
                    .unwrap_or_default();
                let rgba8 = RGBA8 {
                    r: rgb8.r,
                    g: rgb8.g,
                    b: rgb8.b,
                    a: transparency
                        .and_then(|trns| match trns {
                            tRNS::Y { y } => y.to_be_bytes().get(index).copied(),
                            tRNS::RGB { .. } => trns.rgb_to_index().unwrap().get(index).copied(),
                            tRNS::Index { data } => data.get(index).copied(),
                        })
                        .unwrap_or(0xFF),
                };
                final_storage[(y * ihdr.width + x) as usize] = rgba8;
            })?
        }
    }
    //
    Ok((final_storage, ihdr.width, ihdr.height))
}

fn y1_to_rgba8(y1: u8) -> RGBA8 {
    let y2 = y1 | (y1 << 1);
    y2_to_rgba8(y2)
}

fn y2_to_rgba8(y2: u8) -> RGBA8 {
    let y4 = y2 | (y2 << 2);
    y4_to_rgba8(y4)
}

fn y4_to_rgba8(y4: u8) -> RGBA8 {
    let y8 = y4 | (y4 << 4);
    y8_to_rgba8(y8)
}

fn y8_to_rgba8(y8: u8) -> RGBA8 {
    RGBA8 {
        r: y8,
        g: y8,
        b: y8,
        a: 0xFF,
    }
}

fn rgb8_to_rgba8(rgb8: RGB8) -> RGBA8 {
    RGBA8 {
        r: rgb8.r,
        g: rgb8.g,
        b: rgb8.b,
        a: 0xFF,
    }
}

fn rgba16_to_rgba8(rgba16: [u16; 4]) -> RGBA8 {
    RGBA8 {
        r: (rgba16[0] >> 8) as u8,
        g: (rgba16[1] >> 8) as u8,
        b: (rgba16[2] >> 8) as u8,
        a: (rgba16[3] >> 8) as u8,
    }
}

fn rgb16_to_rgba8(rgb16: [u16; 3]) -> RGBA8 {
    RGBA8 {
        r: (rgb16[0] >> 8) as u8,
        g: (rgb16[1] >> 8) as u8,
        b: (rgb16[2] >> 8) as u8,
        a: 0xFF,
    }
}
