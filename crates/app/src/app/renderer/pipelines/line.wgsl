struct InstanceInput {
    @location(0) position_a: vec2<f32>,
    @location(1) position_b: vec2<f32>,
    @location(2) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) @interpolate(flat) color: vec3<f32>,
};

struct CameraUniform {
    center: vec2<f32>,
    scale: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    instance: InstanceInput
) -> VertexOutput {
    let half_width = 0.2;

    let vector = instance.position_b - instance.position_a;
    let width_vector = normalize(vec2(-vector.y, vector.x)) * half_width;

    let line_vec = select(vec2(0.), vector, bool(vertex_index & 1u));
    let width_vec = select(-width_vector, width_vector, bool(vertex_index & 2u));
    let vertex_pos = instance.position_a + width_vec + line_vec;

    var out: VertexOutput;
    let view_pos = (vertex_pos - camera.center) * camera.scale;
    out.clip_position = vec4<f32>(view_pos, 0.0, 1.0);
    out.color = instance.color.rgb;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
