@group(0) @binding(0) var texture_ref: texture_2d<f32>;
@group(0) @binding(1) var sampler_ref: sampler;

// stores all transforms(up to 128mb)
// each index can have its own transform
@group(1) @binding(0) var<storage, read> models: array<mat4x4<f32>>; 

// camera movement
@group(2) @binding(0) var<uniform> view_projection: mat4x4<f32>;

struct VertexInput {
	@location(0) position : vec3<f32>,
};

struct VertexOutput {
	@builtin(position) position : vec4<f32>,
	@location(0) uv : vec2<f32>,
};

// stores index per shader for transforms + ...
struct PushConstants {
	model_index : u32,
};

var<push_constant> push : PushConstants;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
	let model = modes[push.model_index];

	var out : VertexOutput;
	out.position = view_projection * models * vec4<f32>(in.position, 1.0);
	out.uv = vec2<f32>(0.0, 0.0);

	return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
	return textureSample(texture_ref, sampler_ref, in.uv);
}
