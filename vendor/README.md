# Vendored crates

- `epaint` 0.36.2 and `egui-wgpu` 0.36.2, from crates.io (https://github.com/emilk/egui), licensed
  MIT OR Apache-2.0 like egui. Patched in through `[patch.crates-io]` for subpixel (ClearType-style)
  interface text: glyphs are rasterized at three times the horizontal resolution and filtered into
  one coverage per colour channel (`TextOptions::subpixel_text`), text vertices are marked, and the
  wgpu renderer blends each channel on its own (dual-source blending). Changes are marked `LCD:`.
  When the adapter can't do dual-source blending the text falls back to grey anti-aliasing.
