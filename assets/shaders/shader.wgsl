@group(0) @binding(0) var texture_ref: texture_2d<f32>;
@group(0) @binding(1) var sampler_ref: sampler;

// stores all transforms(up to 128mb)
// each index can have its own transform
@group(1) @binding(0) var<storage, read> models: array<mat4x4<f32>>; 

// camera movement
@group(2) @binding(0) var<uniform> view_projection: mat4x4<f32>;

struct Vertex {
	@location(0) position: vec3<f32>,
};

struct VertexPayload {
	@builtin(position) position: vec4<f32>,
	@location(0) uv: vec2<f32>,
};

struct Immediates {
	model_index: u32,
}

var<immediate> immediates: Immediates;

@vertex
fn vs_main(
	vertex: Vertex,
) -> VertexPayload {
	var out: VertexPayload;
	out.position = view_projection * models[immediates.model_index] * vec4<f32>(vertex.position, 1.0f);
    out.uv = vec2<f32>(0.5 * (vertex.position.x + 1f), -0.5 * (vertex.position.y + 1f));
	return out;
}

@fragment 
fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
    return textureSample(texture_ref, sampler_ref, in.uv);
}
