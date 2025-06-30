struct Uniforms {
    center: vec2<f32>,
    range: vec2<f32>,
    max_iter: i32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(1) @binding(0) var storage_texture: texture_storage_2d<rgba8unorm, write>;

fn hsv2rgb(hsv: vec3<f32>) -> vec3<f32> {
    let c = hsv.z * hsv.y;
    let h = hsv.x * 6.0;
    let x = c * (1.0 - abs(h % 2.0 - 1.0));
    var rgb: vec3<f32>;

    if (h < 1.0) {
        rgb = vec3<f32>(c, x, 0.0);
    } else if (h < 2.0) {
        rgb = vec3<f32>(x, c, 0.0);
    } else if (h < 3.0) {
        rgb = vec3<f32>(0.0, c, x);
    } else if (h < 4.0) {
        rgb = vec3<f32>(0.0, x, c);
    } else if (h < 5.0) {
        rgb = vec3<f32>(x, 0.0, c);
    } else {
        rgb = vec3<f32>(c, 0.0, x);
    }

    return rgb + vec3<f32>(hsv.z - c);
}

@compute @workgroup_size(8, 8)
fn main_compute(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dims = textureDimensions(storage_texture);
    if (global_id.x >= dims.x || global_id.y >= dims.y) { return; }

    let normalized_coords = vec2<f32>(f32(global_id.x), f32(global_id.y)) / vec2<f32>(dims);
    let complex_coords = uniforms.center + (normalized_coords - 0.5) * uniforms.range;
    let c = complex_coords;

    let max_iter = uniforms.max_iter;
    var z = vec2<f32>(0.0, 0.0);
    var i = 0;

    while (dot(z, z) < 4.0 && i < max_iter) {
        z = vec2<f32>(
            z.x * z.x - z.y * z.y + c.x,
            2.0 * z.x * z.y + c.y
        );
        i = i + 1;
    }

    var color: vec4<f32>;
    if (i == max_iter) {
        color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    } else {
        let i_f = f32(i);
        let hue = (i_f / 64.0) % 1.0;
        let saturation = 0.8;
        let value = i_f / f32(max_iter);
        color = vec4<f32>(hsv2rgb(vec3(hue, saturation, value)), 1.0);
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
