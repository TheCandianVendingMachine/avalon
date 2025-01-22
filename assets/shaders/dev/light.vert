#version 430

struct lightData {
    vec4 colour;
    vec3 position;
};

layout(location=0) uniform mat4 view;
layout(location=1) uniform mat4 projection;
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

    vec3 cameraDir = (vec4(0, 0, 1, 0) * view).xyz;
    vec3 lightCameraDir = light.position - view[3].xyz;
    vec3 projectedVertex = cameraDir * dot(lightCameraDir, cameraDir) / dot(cameraDir, cameraDir);

    float distance = length(projectedVertex);
    float scale = 1.0 / distance;

    vec4 cameraRelative = view * vec4(vec3(position, 0.0), 1.0);
    gl_Position = vec4(scale * (light.position + cameraRelative.xyz), cameraRelative.w);
}
