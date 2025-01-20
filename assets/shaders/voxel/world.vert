#version 430

layout(location=0) uniform ivec2 uScreenSize;

const float vertices[12] = float[](
    // top left
    -1.0, -1.0,
    1.0, -1.0,
    -1.0, 1.0,
    // bottom right
    1.0, -1.0,
    1.0, 1.0,
    -1.0, 1.0
);

out flat ivec2 screenSize;

void main() {
    screenSize = uScreenSize;
    vec2 position = vec2(
        vertices[gl_VertexID * 2 + 0],
        vertices[gl_VertexID * 2 + 1]
    );
    gl_Position = vec4(position, 0.0, 1.0);
}
