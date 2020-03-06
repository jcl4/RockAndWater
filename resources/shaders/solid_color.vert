// shader.vert
#version 450

layout(location=0) in vec3 vert_pos;

layout(location=0) out vec4 frag_color;

layout(set=0, binding=0)
uniform Uniforms {
	vec4 color;
}

void main() {
    // Changed
    frag_color = color;
    gl_Position = vec4(a_position, 1.0);
}

