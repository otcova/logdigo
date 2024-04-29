struct InstanceInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
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
    let vertex_pos = vec3(x, y, 0.0);

    var out: VertexOutput;
    out.clip_position = vec4<f32>(vertex_pos + instance.position, 1.0);
    out.color = instance.color;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
