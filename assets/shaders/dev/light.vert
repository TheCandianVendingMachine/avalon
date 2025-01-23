#version 430
#define INF (1.0 / 0.0)

struct lightData {
    vec4 colour;
    vec3 position;
};

layout(location=0) uniform ivec2 screenSize;
layout(location=1) uniform mat4 view;
layout(location=3) uniform mat4 projection;
layout(location=5) uniform mat3 projectionTick;
layout(std430, binding=0) readonly buffer storage0 {
    lightData lights[];
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
    lightData light = lights[gl_InstanceID];
    vec2 position = vec2(
        vertices[gl_VertexID * 2 + 0],
        vertices[gl_VertexID * 2 + 1]
    );
    texCoords = 0.5 * position + 0.5;
    lightColour = light.colour;

    float focal = 1.0;

    float n = 0.0;
    float a = float(screenSize.y) / float(screenSize.x);
    mat4 projection = mat4(
        vec4(focal, 0.0, 0.0, 0.0),
        vec4(0.0, -focal / a, 0.0, 0.0),
        vec4(0.0, 0.0, -1.0, -1.0),
        vec4(0.0, 0.0, -2.0 * n, 0.0)
    );

    vec4 vertexPosition = vec4(vec3(1.0, -1.0, -1.0) * light.position + vec3(vec2(0.5, 0.5) * position, 0.0), 1.0);
    gl_Position = projection * view * vertexPosition;
}
