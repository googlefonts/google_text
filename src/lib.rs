//! Reexporting Google Fonts crates

pub extern crate font_types;
pub extern crate read_fonts;

#[cfg(feature = "skrifa")]
pub extern crate skrifa;

#[cfg(feature = "write-fonts")]
pub extern crate write_fonts;

#[cfg(feature = "harfrust")]
pub extern crate harfrust;

/// This fn will only compile if the exported crates are compatibile
#[allow(dead_code)]
fn ensure_versions_are_compatible() {
    #[cfg(feature = "write-fonts")]
    {
        use read_fonts::FontRead;
        let font_data = read_fonts::FontData::new(&[]);
        // this won't compile if write-fonts is expecting a different version of read fonts
        let _ = write_fonts::tables::head::Head::read(font_data);
    }

    #[cfg(feature = "skrifa")]
    {
        fn skrifa_compat_check(_: skrifa::raw::FontData) {}
        // won't compile if skrifa has a different version of read-fonts
        let font_data = read_fonts::FontData::new(&[]);
        skrifa_compat_check(font_data);
    }
    #[cfg(feature = "harfrust")]
    {
        // won/t compile unless harfrust is using latest read_fonts
        let _: Result<harfrust::FontRef, _> = read_fonts::FontRef::new(&[]);
    }
}
