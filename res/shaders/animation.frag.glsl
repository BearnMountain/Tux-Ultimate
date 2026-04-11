#version 430 core

layout (location = 0) in vec2 in_uv; // texure coords

layout (location = 0) out vec4 color;

layout (set = 2, binding = 0) uniform sampler2D tex_sampler;

void main() {
	color = texture(tex_sampler, in_uv);
}
