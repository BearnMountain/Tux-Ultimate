@group(0) @binding(0) var texture_ref: texture_2d<f32>;
@group(0) @binding(1) var sampler_ref: sampler;
@group(1) @binding(0) var<storage, read> models: array<mat4x4<f32>>; 
@group(2) @binding(0) var<uniform> view_projection: mat4x4<f32>;

struct Vertex {
	@location(0) positon: vec3<f32>,
	@location(1) uv: vec2<f32>,
	@location(2) normal: vec3<f32>,
};

struct VertexPayload {
	@builtin(position) position: vec4<f32>,
	@location(0) uv: vec2<f32>,
};

struct Immediates {
	model_index: u32,
};
var<immediate> immediates: Immediates;

@vertex
fn vs_main(
	@builtin(instance_index) instance_index: u32,
	vertex: Vertex,
) -> VertexPayload {
	var out: VertexPayload;
	let model = models[immediates.model_index];
	out.position = view_projection * model * vec4<f32>(vertex.position, 1.0f);
    out.uv = vertex.uv;
	return out;
}

@fragment 
fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
	var base: vec4 = textureSample((texture_2d, sampler, in.uv));
    return vec4<f32>(base.rgb, base.a);
}
