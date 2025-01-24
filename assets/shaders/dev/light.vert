#version 430
#define INF (1.0 / 0.0)

struct lightData {
    vec4 colour;
    vec3 position;
};

layout(location=1) uniform mat4 view;
layout(location=3) uniform mat4 projection;
layout(std430, binding=0) readonly buffer storage0 {
    lightData lights[];
};

const float vertices[12] = float[](
    // top left
    -0.5, -0.5,
    0.5, -0.5,
    -0.5, 0.5,
    // bottom right
    0.5, -0.5,
    0.5, 0.5,
    -0.5, 0.5
);

out smooth vec2 texCoords;
out flat vec4 lightColour;

void main() {
    lightData light = lights[gl_InstanceID];
    vec2 position = vec2(
        vertices[gl_VertexID * 2 + 0],
        vertices[gl_VertexID * 2 + 1]
    );
    texCoords = position + 0.5;
    lightColour = light.colour;

    vec3 cameraRight = vec3(view[0][0], view[1][0], view[2][0]);
    vec3 cameraUp = vec3(view[0][1], view[1][1], view[2][1]);

    vec3 lightPos = vec3(1.0, -1.0, -1.0) * light.position;
    vec3 billboardSize = vec3(vec2(0.3, 0.3) * position, 0.0);

    vec4 vertexPosition = vec4(lightPos + cameraRight * billboardSize.x + cameraUp * billboardSize.y, 1.0);
    gl_Position = projection * view * vertexPosition;
}
