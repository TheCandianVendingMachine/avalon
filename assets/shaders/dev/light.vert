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

float getPlaneIntersection(in vec3 normal, in vec3 center, in vec3 rayOrigin, in vec3 rayDir) {
    float denom = dot(normal, rayDir);
    if (abs(denom) > 0) {
        float t = dot(center - rayOrigin, normal) / denom;
        if (t >= 0) {
            return t;
        }
    }
    return INF;
}

void main() {
    lightData light = lights[gl_InstanceID];
    vec2 position = vec2(
        vertices[gl_VertexID * 2 + 0],
        vertices[gl_VertexID * 2 + 1]
    );
    texCoords = 0.5 * position + 0.5;
    lightColour = light.colour;

    float focal = 1.0;

    vec3 cameraPos = view[3].xyz;
    vec3 cameraDir = (vec4(0, 0, 1, 0) * view).xyz;

    vec3 lightCameraRelative = light.position + (view * vec4(vec3(position, 0.0), 1.0)).xyz;
    vec3 directionToVertex = normalize(cameraPos - lightCameraRelative);

    float t = getPlaneIntersection(cameraDir, cameraDir * focal, cameraPos, directionToVertex);
    vec3 finalPosition = cameraDir * t;

    gl_Position = vec4(finalPosition, 1.0);
}
