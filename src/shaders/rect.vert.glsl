#version 460

layout (location = 0) in vec3 a_pos;
layout (location = 1) in vec4 a_clr;
layout (location = 0) out vec4 v_clr;

void main() {
	gl_Position = vec4(a_pos, 1.0f);
	v_clr = a_clr;
}
