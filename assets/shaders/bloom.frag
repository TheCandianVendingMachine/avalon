#version 430

layout(location=1) uniform sampler2D scene;
layout(location=3) uniform sampler2D bloom;

in flat ivec2 screenSize;
out vec4 albedo;

void main() {
    vec2 uv = gl_FragCoord.xy / vec2(screenSize);
    vec4 sceneColour = texture(scene, uv);
    vec4 bloomColour = texture(bloom, uv);
    albedo = sceneColour + 0.25 * bloomColour;
}
