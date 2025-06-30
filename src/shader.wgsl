struct Uniforms {
    center: vec2<f32>,
    range: vec2<f32>,
    max_iter: i32,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(1) @binding(0) var storage_texture: texture_storage_2d<rgba8unorm, write>;

fn mandelbrot_smooth(c: vec2<f32>, max_iter: i32) -> f32 {
    var z = vec2<f32>(0.0, 0.0);
    for (var i = 0; i < max_iter; i = i + 1) {
        if (dot(z, z) > 16.0) {
            let log_zn = log(dot(z,z)) / 2.0;
            let nu = log(log_zn / log(2.0)) / log(2.0);
            return f32(i) + 1.0 - nu;
        }
        z = vec2<f32>(
            z.x * z.x - z.y * z.y + c.x,
            2.0 * z.x * z.y + c.y
        );
    }
    return f32(max_iter);
}

fn hsv2rgb(hsv: vec3<f32>) -> vec3<f32> {
    let c = hsv.z * hsv.y;
    let h = hsv.x * 6.0;
    let x = c * (1.0 - abs(h % 2.0 - 1.0));
    var rgb: vec3<f32>;

    if (h < 1.0) { rgb = vec3<f32>(c, x, 0.0); }
    else if (h < 2.0) { rgb = vec3<f32>(x, c, 0.0); }
    else if (h < 3.0) { rgb = vec3<f32>(0.0, c, x); }
    else if (h < 4.0) { rgb = vec3<f32>(0.0, x, c); }
    else if (h < 5.0) { rgb = vec3<f32>(x, 0.0, c); }
    else { rgb = vec3<f32>(c, 0.0, x); }

    return rgb + vec3<f32>(hsv.z - c);
}

fn colorize_rainbow_gradient(n: f32, max_iter: i32) -> vec4<f32> {
    if (n >= f32(max_iter)) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    let hue = fract(n / 256.0);

    let saturation = 0.9;

    let value = 1.0;

    let rgb = hsv2rgb(vec3(hue, saturation, value));
    return vec4(rgb, 1.0);
}


@compute @workgroup_size(8, 8, 1)
fn main_compute(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let dims = vec2<f32>(textureDimensions(storage_texture));
    let frag_coord = vec2<f32>(global_id.xy);

    if (frag_coord.x >= dims.x || frag_coord.y >= dims.y) {
        return;
    }

    let norm_coord = (frag_coord / dims) - 0.5;
    let c = uniforms.center + vec2<f32>(norm_coord.x, -norm_coord.y) * uniforms.range;

    let n_smooth = mandelbrot_smooth(c, uniforms.max_iter);
    let color = colorize_rainbow_gradient(n_smooth, uniforms.max_iter);

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
    out.tex_coords = vec2<f32>(f32(in_vertex_index % 2u), 1.0 - f32(in_vertex_index / 2u));
    return out;
}

@fragment
fn main_fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
