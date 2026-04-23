#version 450

layout(location = 0) in vec3 in_normal;
layout(location = 1) in vec2 in_uv;

layout(location = 0) out vec4 out_color;

void main() {
    vec3  N    = normalize(in_normal);
    vec3  L    = normalize(vec3(1.0, 2.0, 1.0));
    float diff = max(dot(N, L), 0.15);
    out_color  = vec4(vec3(diff), 1.0);
}
