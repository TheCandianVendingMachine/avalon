#version 430

in smooth vec3 fPosition;
in smooth vec3 fNormal;
in smooth vec3 fTangent;
in smooth vec4 fColour;

out vec4 albedo;
out vec3 normal;
out vec3 tangent;
out vec3 position;

void main() {
    albedo = fColour;
    position = fPosition;
    normal = fNormal;
    tangent = fTangent;
}
