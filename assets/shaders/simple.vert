#version 460

layout(location = 0) in vec2 pos;
layout(location = 1) in vec4 color;

layout(location = 0) out VertexData {
    vec4 color;
} vertex;


void main() {
    vertex.color = color;

    gl_Position = vec4(pos, 0., 1.);
}
