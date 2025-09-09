# google_text

Metacrate for Google Fonts text processing libraries.

## overview

This crate reexports compatible versions of Google Font's core text processing
crates, which often have interdependencies and live in different repositories.
By using this crate, you ensure that you are always using the latest set of
compatible libraries.

This is _only_ useful if you are using multiple high-level crates
together, such as `write-fonts` and `skrifa`. If you only need to use one of
these, you should just use it directly.

From lowest to highest level, the reexported crates are:

- [`font-types`][] For basic data types
- [`read-fonts`][] For parsing and font files and exposing their contents
- [`write-fonts`][] For modifying and writing font files
- [`skrifa`][] A higher level API for accessing font data
- [`HarfRust`][] A Rust port of [`HarfBuzz`][], a [shaping engine][]

## features

This crate _always_ exports `font-types` and `read-fonts` (which are
dependencies of everything else) and _by default_ it exports all of the crates
above. If you only need some of them (for instance you only need `write-fonts`
and `skrifa`, or only `skrifa` and `HarfRust`) you can select them manually:

```toml
[dependencies]
google-text = {version = "0.1", default-features = false, features =
["write-fonts", "skrifa"] }
```



[`font-types`]: https://github.com/googlefonts/fontations/tree/main/font-types
[`read-fonts`]: https://github.com/googlefonts/fontations/tree/main/read-fonts
[`write-fonts`]: https://github.com/googlefonts/fontations/tree/main/write-fonts
[`skrifa`]: https://github.com/googlefonts/fontations/tree/main/skrifa
[`HarfRust`]: https://github.com/harfbuzz/harfrust/blob/main/Cargo.toml
[`HarfBuzz`]: https://harfbuzz.github.io
[shaping engine]: https://en.wikipedia.org/wiki/Text_shaping
