struct InstanceInput {
    @location(0) position: vec2<f32>,
    @location(1) size: vec2<u32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) @interpolate(flat) color: vec3<f32>,
    // Centered model vertex position coordinates
    @location(1) coords: vec2<f32>,
    @location(2) @interpolate(flat) half_size: vec2<f32>,
    // pixels per unit
    @location(3) @interpolate(flat) resolution: f32,
};

struct CameraUniform {
    center: vec2<f32>,
    clipping_scale: vec2<f32>,
    pixel_scale: f32,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    instance: InstanceInput
) -> VertexOutput {
    let size = vec2<f32>(instance.size);
    let half_size = size / 2.0;

    let x = select(0., size.x, bool(vertex_index & 1u));
    let y = select(0., size.y, bool(vertex_index & 2u));
    let model_vertex = vec2(x, y);
    let world_vertex = model_vertex + instance.position;
    let view_vertex = (world_vertex - camera.center) * camera.clipping_scale;



    var out: VertexOutput;
    out.clip_position = vec4<f32>(view_vertex, 0.0, 1.0);
    out.color = instance.color.rgb;
    out.coords = model_vertex - half_size;
    out.half_size = half_size;
    out.resolution = camera.pixel_scale;
    return out;
}

fn squircle(pos: vec2<f32>, half_size: vec2<f32>, half_radius: f32) -> f32 {
    let p = abs(pos) / half_size;
    let w = pow(p, half_size / half_radius);
    return 1.0 - (w.x + w.y);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let half_radius = 0.5 / 2.;
    let outter_step = 2. / in.resolution;
    let inner_step = 1. / in.resolution;
    let stroke_width = 0.3;

    let distance = squircle(in.coords, in.half_size, half_radius);
    let outter_mask = smoothstep(-outter_step, outter_step, distance);
    let color = mix(vec3(1.), in.color.rgb, smoothstep(-inner_step, inner_step, distance - stroke_width));

    return vec4<f32>(color * outter_mask, outter_mask);
}
