//! Everything related to text, fonts, text layout, cursors etc.

pub mod cursor;
mod font;
mod fonts;
mod index;
mod text_layout;
mod text_layout_types;

pub use {
    fonts::{
        FontData, FontDefinitions, FontFamily, FontId, FontInsert, FontPriority, FontTweak,
        FontVariationAxis, Fonts, FontsImpl, FontsView, HintingTarget, InsertFontFamily,
        SmoothHinting,
    },
    index::{ByteIndex, ByteRange, ByteRangeExt, CharIndex, CharRange, CharRangeExt},
    text_layout::*,
    text_layout_types::*,
};

/// Suggested character to use to replace those in password text fields.
pub const PASSWORD_REPLACEMENT_CHAR: char = '•';

/// Controls how we render text
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TextOptions {
    /// Maximum size of the font texture.
    pub max_texture_side: usize,

    /// Controls how to convert glyph colors when writing to the font atlas.
    pub color_transfer_function: crate::FontColorTransferFunction,

    /// Whether to enable font hinting
    ///
    /// (round some font coordinates to pixels for sharper text).
    ///
    /// Default is `true`.
    pub font_hinting: bool,

    /// Enable sub-pixel binning for glyphs.
    ///
    /// Sub-pixel binning renders each glyph at up to four fractional horizontal offsets,
    /// giving more even kerning at the cost of more atlas space.
    ///
    /// It also lead to text looking more blurry.
    ///
    /// This is always disabled for CJK characters (which have too many unique glyphs).
    ///
    /// Can be overridden per font with [`FontTweak::subpixel_binning`].
    ///
    /// Default: `true`.
    pub subpixel_binning: bool,
}

/// LCD: whether glyphs are rasterized for subpixel text (see [`set_subpixel_text`]).
static SUBPIXEL_TEXT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// LCD: rasterize glyphs for subpixel (ClearType-style) rendering on an RGB-striped screen: one
/// coverage per colour channel, from the outline at three times the horizontal resolution. The
/// renderer blends each channel on its own where it can (egui-wgpu with dual-source blending) and
/// draws the mean coverage otherwise. Meant for low-DPI screens; set it before the fonts are
/// first used (glyphs already in the atlas keep how they were drawn). Off by default.
///
/// A global switch rather than a [`TextOptions`] field: egui destructures `TextOptions`, and
/// this vendored copy doesn't patch egui.
pub fn set_subpixel_text(on: bool) {
    SUBPIXEL_TEXT.store(on, std::sync::atomic::Ordering::Relaxed);
}

/// LCD: see [`set_subpixel_text`].
pub fn subpixel_text() -> bool {
    SUBPIXEL_TEXT.load(std::sync::atomic::Ordering::Relaxed)
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            max_texture_side: 2048, // Small but portable
            color_transfer_function: crate::FontColorTransferFunction::default(),
            font_hinting: true,
            subpixel_binning: true,
        }
    }
}
