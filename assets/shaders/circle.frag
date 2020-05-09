#version 450

layout(location = 0) in VertexData {
    vec4 color;
    vec2 rel;
} vertex;

layout(location = 0) out vec4 out_color;

void main() {
    float sqr = dot(vertex.rel, vertex.rel);
    if (sqr <= 1) {
        out_color = vertex.color;
    } else {
        discard;
    }
}
