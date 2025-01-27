#version 430

layout(location=1) uniform sampler2D colour;

in flat ivec2 screenSize;
out vec4 albedo;

void main() {
    vec2 uv = gl_FragCoord.xy / vec2(screenSize);
    albedo = texture(colour, uv);
    if (albedo.a == 0.0) {
        discard;
    }
}
