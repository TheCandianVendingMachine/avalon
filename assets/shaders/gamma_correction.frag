#version 430

layout(location=1) uniform sampler2D texture;

out vec4 fColor;

void main() {
    const float gamma = 2.2;
    vec3 rgb = texelFetch(texture, ivec2(gl_FragCoord.xy), 0).rgb;
    vec3 corrected = pow(rgb, vec3(gamma));
    fColor = vec4(corrected, 1.0);
}
