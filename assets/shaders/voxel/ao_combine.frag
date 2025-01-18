#version 430

layout(location=1) uniform sampler2D lightBuffer;
layout(location=2) uniform sampler2D aoBuffer;
layout(location=3) uniform sampler2D albedoBuffer;

out vec4 fColor;

void main() {
    vec3 light = texelFetch(lightBuffer, ivec2(gl_FragCoord.xy), 0).rgb;
    vec3 ao = texelFetch(aoBuffer, ivec2(gl_FragCoord.xy), 0).rgb;
    vec3 albedo = texelFetch(albedoBuffer, ivec2(gl_FragCoord.xy), 0).rgb;

    fColor = vec4(albedo * (light + ao), 1.0);
}

