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

@vertex
fn vs_main(
	@builtin(instance_index) instance_index: u32,
	vertex: Vertex,
) -> VertexPayload {
	var out: VertexPayload;
	let world_pos =	models[0] * vec4<f32>(vertex.position, 1.0f);
	out.position = view_projection * world_pos;
    out.uv = vec2<f32>(0.5 * (vertex.position.x + 1f), -0.5 * (vertex.position.y + 1f));
	return out;
}

@fragment 
fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
    return vec4(models[0][3].xy, 1.0, 1.0) * textureSample(texture_ref, sampler_ref, in.uv);
}
