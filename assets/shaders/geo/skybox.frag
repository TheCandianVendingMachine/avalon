#version 430
#define PI (3.1415926535)

layout(location=0) uniform sampler2D skydome;
layout(location=2) uniform mat4 view;
layout(location=4) uniform mat4 projection;

in flat ivec2 screenSize;

out vec3 fColor;
out vec4 fBloom;

void main() {
    vec2 uv = gl_FragCoord.xy / vec2(screenSize) * 2.0 - 1.0;

    vec3 cameraDir = (vec4(0, 0, 1, 0) * view).xyz;
    vec3 rayDir = vec3(projection[0][0] * uv.x, -uv.y / projection[1][1], projection[0][0]);
    rayDir = normalize((view * vec4(rayDir, 0)).xyz);

    vec2 samplePos = vec2(rayDir.z / (1.0 - rayDir.y), -rayDir.x / (1.0 - rayDir.y)) / (0.5 * PI);

    fColor = 1.0 * texture(skydome, samplePos).rgb;
    fBloom = vec4(0.0);
}

