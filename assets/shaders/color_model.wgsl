@group(0) @binding(0) var<uniform> color: vec4<f32>;
@group(1) @binding(0) var<storage, read> models: array<mat4x4<f32>>; 
@group(2) @binding(0) var<uniform> view_projection: mat4x4<f32>;

struct Vertex {
	@location(0) positon: vec3<f32>,
	@location(1) uv: vec2<f32>,
	@location(2) normal: vec3<f32>,
};

struct VertexPayload {
	@builtin(position) position: vec4<f32>,
	@location(0) normal: vec2<f32>,
};

struct Immediates {
	model_index: u32,
};

var<immediate> immediates: Immediates;

@vertex
fn vs_main(
	vertex: Vertex,
) -> VertexPayload {
	var out: VertexPayload;
	let model = models[immediates.model_index];
	out.position = view_projection * model * vec4<f32>(vertex.position, 1.0f);
    out.normal = (model * vec4<f32>(vertex.normal), 0.0).xyz;
	return out;
}

@fragment 
fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
    return vec4<f32>(color.rgb, 1.0);
}
