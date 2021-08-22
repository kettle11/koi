# kolor

kolor is a crate for doing conversions between color spaces and helps with some other color math.

kolor is intended for use in games or other interactive visual applications,
where it can help implement correct color management, wide color gamut rendering and tonemapping.

# Example

```rust
let conversion = kolor::ColorConversion::new(
    kolor::spaces::SRGB,
    kolor::spaces::ACES_CG,
);
let srgb_color = kolor::Vec3::new(0.25, 0.5, 0.75);
let aces_cg_color = conversion.convert(srgb_color);
```

# Design
kolor aims to supports all color spaces and color models which use 3-component vectors,
such as RGB, LAB, XYZ, HSL, LMS and more.

In the spirit of keeping things simple, kolor uses a single type, [ColorConversion](https://docs.rs/kolor/latest/kolor/struct.ColorConversion.html), to represent
a color conversion between any supported color spaces.

For more details on design and implementation, please have a look at the [module docs.](https://docs.rs/kolor/latest/kolor/index.html)

# no_std support
kolor supports `no_std` by disabling the default-enabled `std` feature and enabling the `libm` feature.


### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT).

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

PLEASE NOTE that some dependencies may be licensed under other terms. These are listed in [deny.toml](deny.toml) under licenses.exceptions on a best-effort basis, and are validated in every CI run using [cargo-deny](https://github.com/EmbarkStudios/cargo-deny).