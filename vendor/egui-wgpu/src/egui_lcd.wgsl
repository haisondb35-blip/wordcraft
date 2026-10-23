// LCD: subpixel (ClearType-style) text, appended to egui.wgsl when the device can do dual-source
// blending (the source gets `enable dual_source_blending;` in front).
//
// A text fragment's texel holds one coverage per colour channel (epaint's subpixel glyphs; plain
// glyphs have the same coverage in all three). Each channel is blended on its own:
// `dst = color + dst · (1 − blend)`, with `blend` the coverage times the text's alpha. Everything
// else blends as before (`blend` is its alpha in every channel).

struct LcdOutput {
    @location(0) @blend_src(0) color: vec4<f32>,
    @location(0) @blend_src(1) blend: vec4<f32>,
};

fn lcd_output(in: VertexOutput, tex_gamma: vec4<f32>) -> LcdOutput {
    var out: LcdOutput;
    if in.text == 1u {
        let coverage = tex_gamma.rgb;
        out.color = vec4<f32>(in.color.rgb * coverage, in.color.a * max(coverage.r, max(coverage.g, coverage.b)));
        out.blend = vec4<f32>(in.color.a * coverage, out.color.a);
    } else {
        out.color = in.color * tex_gamma;
        out.blend = vec4<f32>(out.color.a);
    }
    if r_locals.dithering == 1 {
        out.color = vec4<f32>(dither_interleaved(out.color.rgb, 256.0, in.position), out.color.a);
    }
    return out;
}

@fragment
fn fs_main_linear_framebuffer_lcd(in: VertexOutput) -> LcdOutput {
    var out = lcd_output(in, sample_texture(in));
    out.color = vec4<f32>(linear_from_gamma_rgb(out.color.rgb), out.color.a);
    return out;
}

@fragment
fn fs_main_gamma_framebuffer_lcd(in: VertexOutput) -> LcdOutput {
    return lcd_output(in, sample_texture(in));
}
