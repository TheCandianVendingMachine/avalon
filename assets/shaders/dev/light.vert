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
    vec2 vertexPos = vec2(
        vertices[gl_VertexID * 2 + 0],
        vertices[gl_VertexID * 2 + 1]
    );
    texCoords = vertexPos + vec2(0.5);
    lightColour = light.colour;

    vec3 viewPosition = vec3(-view[3][0], view[3][1], view[3][2]);
    mat4 viewRotation = inverse(mat4(
        view[0],
        view[1],
        view[2],
        vec4(0.0, 0.0, 0.0, 1.0)
    ));

    vec3 cameraRight = vec3(viewRotation[0][0], viewRotation[1][0], viewRotation[2][0]);
    vec3 cameraUp = vec3(viewRotation[0][1], viewRotation[1][1], viewRotation[2][1]);

    vec3 lightPos = viewPosition + vec3(-1.0, -1.0, -1.0) * light.position;
    vec2 billboardSize = vec2(0.7, 0.7);

    vec4 vertexPosition = vec4(
        lightPos +
        cameraRight * billboardSize.x * vertexPos.x +
        cameraUp * billboardSize.y * vertexPos.y,
        1.0
    );

    gl_Position = projection * viewRotation * vertexPosition;
}
