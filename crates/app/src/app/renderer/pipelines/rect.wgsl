struct InstanceInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) @interpolate(flat) color: vec3<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    instance: InstanceInput
) -> VertexOutput {

    let x = select(0., 1., bool(vertex_index & 1u));
    let y = select(0., 1., bool(vertex_index & 2u));
    let vertex_pos = vec2(x, y) + instance.position;

    var out: VertexOutput;
    out.clip_position = vec4<f32>(vertex_pos, 0.0, 1.0);
    out.color = instance.color.rgb;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
