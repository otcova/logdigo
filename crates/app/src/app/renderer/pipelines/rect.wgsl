struct InstanceInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
    @location(2) size: vec2<u32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texture_coords: vec2<f32>,
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

    let x = f32(vertex_index & 1u);
    let y = f32((vertex_index & 2u) >> 1);
    let model_vertex = vec2(x, y);
    let world_vertex = model_vertex * size + instance.position.xy;
    let view_vertex = (world_vertex - camera.center) * camera.clipping_scale;

    var out: VertexOutput;
    out.clip_position = vec4<f32>(view_vertex, instance.position.z, 1.0);
    out.texture_coords = model_vertex;
    return out;
}

@group(1) @binding(0)
var texture: texture_2d<f32>;
@group(1) @binding(1)
var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, in.texture_coords);
}
