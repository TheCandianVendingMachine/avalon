#version 430

layout(location=1) uniform sampler2D albedo;
layout(location=2) uniform sampler2D light;

out vec4 fColor;

void main() {
    vec4 albedo = texelFetch(albedo, ivec2(gl_FragCoord.xy), 0);
    vec3 light = texelFetch(light, ivec2(gl_FragCoord.xy), 0).rgb;

    fColor = vec4(albedo.rgb * light, albedo.a);
}

