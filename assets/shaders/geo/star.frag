#version 430

in smooth vec4 fColour;

out vec3 albedo;
out vec4 bloom;

void main() {
    albedo = fColour.rgb;
    bloom = vec4(2.0 * fColour.rgb, 1.0);
}
