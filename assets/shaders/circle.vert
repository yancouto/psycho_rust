#version 450

layout(location = 0) in vec2 pos;
layout(location = 1) in vec4 color;
layout(location = 2) in vec2 rel;

layout(location = 0) out VertexData {
    vec4 color;
    vec2 rel;
} vertex;


void main() {
    vertex.color = color;
    vertex.rel = rel;

    gl_Position = vec4(pos, 0., 1.);
}
