#version 430

struct lightData {
    mat4 model;
    vec4 colour;
};

layout(location=0) uniform mat4 view;
layout(binding=0, std430) readonly buffer LightBuffer {
    lightData lightBuffer[];
};

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

out smooth vec2 texCoords;
out flat vec4 lightColour;

void main() {
    lightData light = lightBuffer[gl_InstanceID];
    vec2 position = vec2(
        vertices[gl_VertexID * 2 + 0],
        vertices[gl_VertexID * 2 + 1]
    );
    texCoords = position;
    lightColour = vec4(1.0, 0.1, 0.7, 0.5);
    gl_Position = view * light.model * vec4(position, 0.0, 1.0);
}
