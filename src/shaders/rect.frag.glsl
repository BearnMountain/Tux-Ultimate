#version 460

layout (location = 0) in vec4 v_clr;
layout (location = 0) out vec4 FragColor;

void main() {
	FragColor = v_clr;
}
