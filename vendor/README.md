# Vendored crates

- `epaint`, `egui`, `egui-winit`, `egui-wgpu`, `egui_glow`, `egui_kittest` and `eframe` 0.37.0, from
  the storytold egui fork (https://github.com/storytold/egui, rev 6d518fc7) that upstream WordCraft
  builds with, as `cargo vendor` writes them; licensed MIT OR Apache-2.0 like egui. Only `epaint`
  and `egui-wgpu` are changed; the others are here so that every crate uses this one `epaint`. Patched in through `[patch.crates-io]` for subpixel (ClearType-style)
  interface text: glyphs are rasterized at three times the horizontal resolution and filtered into
  one coverage per colour channel (`TextOptions::subpixel_text`), text vertices are marked, and the
  wgpu renderer blends each channel on its own (dual-source blending). Changes are marked `LCD:`.
  When the adapter can't do dual-source blending the text falls back to grey anti-aliasing.
