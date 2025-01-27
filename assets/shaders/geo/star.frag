#version 430

in smooth vec4 fColour;

out vec3 albedo;
out float bloom;

void main() {
    albedo = fColour.rgb;
    bloom = 1.0;
}
