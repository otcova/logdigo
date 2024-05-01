struct InstanceInput {
    @location(0) position: vec2<f32>,
    @location(1) size: vec2<u32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) @interpolate(flat) color: vec3<f32>,
};

struct CameraUniform {
    min_corner: vec2<f32>,
    max_corner: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    instance: InstanceInput
) -> VertexOutput {

    let x = select(0., f32(instance.size.x), bool(vertex_index & 1u));
    let y = select(0., f32(instance.size.y), bool(vertex_index & 2u));
    let vertex_pos = vec2(x, y) + instance.position;

    let camera_center = (camera.max_corner + camera.min_corner) / 2.;
    let camera_half_size = camera_center - camera.min_corner;
    let view_pos = (vertex_pos - camera_center) / camera_half_size;

    var out: VertexOutput;
    out.clip_position = vec4<f32>(view_pos, 0.0, 1.0);
    out.color = instance.color.rgb;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
