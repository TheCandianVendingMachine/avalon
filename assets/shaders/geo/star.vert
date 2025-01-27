#version 430

in vec3 vPosition;
in vec3 vNormal;
in vec3 vTangent;

layout(location=1) uniform mat4 model;
layout(location=3) uniform mat4 view;
layout(location=5) uniform mat4 projection;

out smooth vec3 fPosition;
out smooth vec3 fNormal;
out smooth vec3 fTangent;
out smooth vec4 fColour;

void main() {
    vec4 modelPosition = model * vec4(vPosition, 1.0);
    modelPosition *= vec4(-1.0, -1.0, -1.0, 1.0);
    gl_Position = projection * view * modelPosition;

    mat3 rotation = mat3(
        view[0].xyz,
        view[1].xyz,
        view[2].xyz
    );

    fPosition = modelPosition.xyz;
    fNormal = rotation * vNormal;
    fTangent = rotation * vTangent;
    fColour = vec4(0.3, 0.5, 0.7, 1.0);
}
