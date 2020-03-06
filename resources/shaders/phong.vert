// shader.vert
#version 450

layout(location=0) in vec3 vert_pos;
layout(location=1) in vec3 vert_color;

layout(location=0) out vec4 frag_color;

void main() {
    frag_color = vec4(vert_color, 1.0);
    gl_Position = vec4(vert_pos, 1.0);
}

