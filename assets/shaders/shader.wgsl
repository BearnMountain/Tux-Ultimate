@group(0) @binding(0) var texture_ref: texture_2d<f32>;
@group(0) @binding(1) var sampler_ref: sampler;

struct Vertex {
	@location(0) position: vec3<f32>,
	@location(1) color: vec3<f32>,
};

struct VertexPayload {
	@builtin(position) position: vec4<f32>,
	@location(0) color: vec3<f32>,
	@location(1) uv: vec2<f32>,
};

@vertex
fn vs_main(vertex: Vertex) -> VertexPayload {
	var out: VertexPayload;
	out.position = vec4<f32>(vertex.position, 1.0f);
	out.color = vertex.color;
    out.uv = vec2<f32>(0.5 * (vertex.position.x + 1f), -0.5 * (vertex.position.y + 1f));
	return out;
}

@fragment 
fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0) * textureSample(texture_ref, sampler_ref, in.uv);
}
