//! Everything related to text, fonts, text layout, cursors etc.

mod caret;
pub mod cursor;
mod face_store;
mod family;
mod font_data;
mod font_definitions;
mod font_face;
mod font_id;
mod font_provider;
mod font_tweak;
mod fonts;
mod galley_cache;
mod glyph_atlas;
mod glyph_rasterizer;
pub mod grapheme;
mod index;
mod line_break;
mod styled_metrics;
mod text_layout;
mod text_layout_types;
mod unicode;

pub use {
    font_data::{Blob, FontData, FontVariationAxis},
    font_definitions::{FontDefinitions, FontInsert, FontPriority, InsertFontFamily},
    font_id::{FontFamily, FontId},
    font_provider::{FallbackRequest, FontProvider},
    font_tweak::{FallbackAlign, FontTweak, HintingTarget, SmoothHinting},
    fonts::{Fonts, FontsView, MAX_GLYPH_SIZE, MissingGlyphPolicy, ViewportKey},
    glyph_rasterizer::{
        GlyphBitmap, GlyphRasterizer, GlyphRasterizerRequest, RasterizedGlyph,
        has_emoji_presentation,
    },
    index::{ByteIndex, ByteRange, ByteRangeExt, CharIndex, CharRange, CharRangeExt},
    line_break::{LineBreak, LineBreakKind, LineBreaker, SimpleLineBreaker},
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
