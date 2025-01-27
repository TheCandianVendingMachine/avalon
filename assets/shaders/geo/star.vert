#version 430

in vec3 vPosition;

struct Random {
    uint s0;
    uint s1;
};

layout(location=1) uniform mat4 view;
layout(location=3) uniform mat4 projection;

out smooth vec4 fColour;

// from https://www.shadertoy.com/view/ssGXDd
float unorm(uint n) { return float(n) * (1.0 / float(0xffffffffU)); }
uint uhash(uint a, uint b) {
    uint x = ((a * 1597334673U) ^ (b * 3812015801U));
    // from https://nullprogram.com/blog/2018/07/31/
    x = x ^ (x >> 16u);
    x = x * 0x7feb352du;
    x = x ^ (x >> 15u);
    x = x * 0x846ca68bu;
    x = x ^ (x >> 16u);
    return x;
}
uint urandom(inout Random rng) {
    uint last = rng.s1;
    uint next = uhash(rng.s0, rng.s1);
    rng.s0 = rng.s1; rng.s1 = next;
    return last;
}
float random(inout Random rng) { return unorm(urandom(rng)); }
vec2 random2(inout Random rng) { return vec2(random(rng),random(rng)); }
float gaussian(inout Random rng, float mu, float sigma) {
    vec2 q = random2(rng);
    float g2rad = sqrt(-2.0 * (log(1.0 - q.y)));
    float z = cos(q.x*6.28318530718) * g2rad;
    return mu + z * sigma;
}

vec3 randomPointSphere(inout Random rng, float radius) {
    float x = gaussian(rng, 0.0, 1.0);
    float y = gaussian(rng, 0.0, 1.0);
    float z = gaussian(rng, 0.0, 1.0);

    if (x == 0.0 && y == 0.0 && z == 0.0) {
        x = 0.001;
        y = 0.001;
        z = 0.001;
    }

    vec3 point = normalize(vec3(x, y, z));
    return point * radius;
}

void main() {
    Random state;
    state.s0 = gl_InstanceID;
    state.s0 ^= state.s0 << 13;
    state.s0 ^= state.s0 >> 17;
    state.s0 ^= state.s0 << 5;
    state.s1 = uhash(gl_InstanceID, state.s0);

    vec3 position = randomPointSphere(state, 350.0);

    state.s1 = uhash(gl_VertexID, state.s1);
    vec3 jitter = randomPointSphere(state, 0.1);

    vec3 viewPosition = vec3(view[0][3], view[1][3], view[2][3]);
    mat4 viewRotation = mat4(
        view[0],
        view[1],
        view[2],
        vec4(0.0, 0.0, 0.0, 1.0)
    );

    vec4 modelPosition = vec4(position + jitter + vPosition, 1.0);
    gl_Position = projection * viewRotation * modelPosition;

    fColour = vec4(1.0, 1.0, 1.0, 1.0);
}
