#version 430

layout(location=1) uniform sampler2D texture;
layout(location=2) uniform vec3 white;

out vec4 fColor;

void main() {
    vec3 colour = texelFetch(texture, ivec2(gl_FragCoord.xy), 0).rgb;

    vec3 numerator = colour * (1.0 + colour / (white * white));
    vec3 denom = 1.0 + colour;
    fColor = vec4(numerator / denom, 1.0);
}
