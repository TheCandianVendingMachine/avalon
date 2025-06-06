#version 430

layout(location=2) uniform sampler2D icon;

in smooth vec2 texCoords;
in flat vec4 lightColour;
out vec4 fColor;

void main() {
    vec4 colour = texture(icon, texCoords);
    if (colour.a == 0.0) {
        discard;
    }
    if (colour.rgb == vec3(1.0)) {
        colour = lightColour;
    }
    fColor = vec4(colour);
}

