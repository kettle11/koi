//! Color constants in here are sourced from Wikipedia or web standards.
//! These colors are a mix of expected standards and fun unusual picks.
use crate::Color;
impl Color {
    /// White in sRGB
    pub const WHITE: Color = Color {
        x: 0.950470,
        y: 1.0000,
        z: 1.08883,
        alpha: 1.0,
    };

    /// Black in sRGB
    pub const BLACK: Color = Color {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        alpha: 1.0,
    };

    /// Red in sRGB
    pub const RED: Color = Color {
        x: 0.412456,
        y: 0.212673,
        z: 0.019334,
        alpha: 1.0,
    };

    /// Green in sRGB
    pub const GREEN: Color = Color {
        x: 0.357576,
        y: 0.715152,
        z: 0.119192,
        alpha: 1.0,
    };

    /// Blue in sRGB
    pub const BLUE: Color = Color {
        x: 0.180437,
        y: 0.072175,
        z: 0.950304,
        alpha: 1.0,
    };

    pub const YELLOW: Color = Color {
        x: 0.7700324,
        y: 0.927825,
        z: 0.13852587,
        alpha: 1.0,
    };
    pub const ORANGE: Color = Color {
        x: 0.54699874,
        y: 0.48175755,
        z: 0.06418133,
        alpha: 1.0,
    };
    pub const PINK: Color = Color {
        x: 0.70869774,
        y: 0.6327434,
        z: 0.64968526,
        alpha: 1.0,
    };
    pub const BROWN: Color = Color {
        x: 0.16764855,
        y: 0.09825001,
        z: 0.03203705,
        alpha: 1.0,
    };
    pub const CYAN: Color = Color {
        x: 0.53801364,
        y: 0.78732723,
        z: 1.0694962,
        alpha: 1.0,
    };
    pub const MAGENTA: Color = Color {
        x: 0.59289384,
        y: 0.2848478,
        z: 0.9696381,
        alpha: 1.0,
    };
    pub const PURPLE: Color = Color {
        x: 0.12798238,
        y: 0.0614874,
        z: 0.2093066,
        alpha: 1.0,
    };
    pub const AZURE: Color = Color {
        x: 0.25632614,
        y: 0.22395226,
        z: 0.9756004,
        alpha: 1.0,
    };
    pub const OCHRE: Color = Color {
        x: 0.3179026,
        y: 0.2614999,
        z: 0.04886362,
        alpha: 1.0,
    };
    pub const MINT: Color = Color {
        x: 0.22820842,
        y: 0.3547034,
        z: 0.29305845,
        alpha: 1.0,
    };
    pub const INTERNATIONAL_ORANGE: Color = Color {
        x: 0.44041428,
        y: 0.26858872,
        z: 0.028653206,
        alpha: 1.0,
    };
    pub const ELECTRIC_INDIGO: Color = Color {
        x: 0.24600193,
        y: 0.105981655,
        z: 0.95337754,
        alpha: 1.0,
    };
    pub const MAJORELLE_BLUE: Color = Color {
        x: 0.20606798,
        y: 0.13390106,
        z: 0.6919497,
        alpha: 1.0,
    };
}

#[test]
fn print_colors() {
    let color = Color::new_from_bytes(255, 255, 0, 255);
    println!("pub const YELLOW: Color = {:#?};", color);

    let color = Color::new_from_bytes(255, 165, 0, 255);
    println!("pub const ORANGE: Color = {:#?};", color);

    let color = Color::new_from_bytes(255, 192, 203, 255);
    println!("pub const PINK: Color = {:#?};", color);

    let color = Color::new_from_bytes(165, 42, 42, 255);
    println!("pub const BROWN: Color = {:#?};", color);

    let color = Color::new_from_bytes(0, 255, 255, 255);
    println!("pub const CYAN: Color = {:#?};", color);

    let color = Color::new_from_bytes(255, 0, 255, 255);
    println!("pub const MAGENTA: Color = {:#?};", color);

    let color = Color::new_from_bytes(128, 0, 128, 255);
    println!("pub const PURPLE: Color = {:#?};", color);

    let color = Color::new_from_bytes(0, 127, 255, 255);
    println!("pub const AZURE: Color = {:#?};", color);

    let color = Color::from_srgb_hex(0xCC7722, 1.0);
    println!("pub const OCHRE: Color = {:#?};", color);

    let color = Color::from_srgb_hex(0x3EB489, 1.0);
    println!("pub const MINT: Color = {:#?};", color);

    let color = Color::from_srgb_hex(0xFF4F00, 1.0);
    println!("pub const INTERNATIONAL_ORANGE: Color = {:#?};", color);

    let color = Color::from_srgb_hex(0x6F00FF, 1.0);
    println!("pub const ELECTRIC_INDIGO: Color = {:#?};", color);

    let color = Color::from_srgb_hex(0x6050DC, 1.0);
    println!("pub const MAJORELLE_BLUE: Color = {:#?};", color);
}
