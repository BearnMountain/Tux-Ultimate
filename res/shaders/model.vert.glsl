#version 450

layout(location = 0) in vec3 in_pos;
layout(location = 1) in vec3 in_normal;
layout(location = 2) in vec2 in_uv;

layout(set = 1, binding = 0) uniform ModelUniforms {
    mat4 mvp;
};

layout(location = 0) out vec3 out_normal;
layout(location = 1) out vec2 out_uv;

void main() {
    out_normal  = in_normal;
    out_uv      = in_uv;
    gl_Position = mvp * vec4(in_pos, 1.0);
}
