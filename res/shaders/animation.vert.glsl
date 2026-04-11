#version 430 core

layout (location = 0) in vec3 pos;
layout (location = 1) in vec3 norm;
layout (location = 2) in vec2 in_uv;
layout (location = 5) in ivec4 bone_ids;
layout (location = 6) in vec4 weights;

uniform mat4 mvp; // mvp calculated on cpu

const int MAX_BONES = 100;
const int MAX_BONE_INFLUENCE = 4; // good enough for 4x4 matrix
uniform mat4 final_bones_matrice[MAX_BONES];

layout (location = 0) out vec2 out_uv;

void main() {
	vec4 net_pos = vec4(0.0f);
	for (int i = 0; i < MAX_BONE_INFLUENCE; i++) {
		if (bone_ids[i] == -1)
			continue;
		if (bone_ids[i] >= MAX_BONES) {
			net_pos = vec4(pos, 1.0f);
			break;
		}

		vec4 local_pos = final_bones_matrice[bone_ids[i]] * vec4(pos, 1.0f);
		net_pos += local_pos * weights[i];
		vec3 local_norm = mat3(final_bones_matrice[bone_ids[i]]) * norm;
	}

	// position relative to bones
	gl_Position = mvp * net_pos; 
	
	// for fragment shader
	out_uv = in_uv; 
}
