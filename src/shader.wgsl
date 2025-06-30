struct Uniforms {
    center: vec2<f32>,
    range: vec2<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(1) @binding(0) var storage_texture: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(8, 8)
fn main_compute(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dims = textureDimensions(storage_texture);
    if (global_id.x >= dims.x || global_id.y >= dims.y) { return; }

    let normalized_coords = vec2<f32>(f32(global_id.x), f32(global_id.y)) / vec2<f32>(dims);
    let complex_coords = uniforms.center + (normalized_coords - 0.5) * uniforms.range;
    let cx = complex_coords.x;
    let cy = complex_coords.y;

    let max_iter = 128;
    var zx = 0.0;
    var zy = 0.0;
    var i = 0;

    while (zx * zx + zy * zy < 4.0 && i < max_iter) {
        let xtemp = zx * zx - zy * zy + cx;
        zy = 2.0 * zx * zy + cy;
        zx = xtemp;
        i = i + 1;
    }
    var color: vec4<f32>;
    if (i == max_iter) {
        color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    } else {
        color = vec4<f32>(f32(i * 5) / 255.0, f32(i * 2) / 255.0, f32(i * 8) / 255.0, 1.0);
    }

    textureStore(storage_texture, global_id.xy, color);
}

@group(0) @binding(0) var t_diffuse: texture_2d<f32>;
@group(0) @binding(1) var s_diffuse: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn main_vertex(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    let x = f32(in_vertex_index % 2u) * 2.0 - 1.0;
    let y = f32(in_vertex_index / 2u) * 2.0 - 1.0;
    out.clip_position = vec4<f32>(x, -y, 0.0, 1.0);
    out.tex_coords = vec2<f32>(f32(in_vertex_index % 2u), f32(in_vertex_index / 2u));
    return out;
}

@fragment
fn main_fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
