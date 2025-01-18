#version 430

#define PI (3.1415926535)

layout(location=1) uniform sampler2D positionBuffer;
layout(location=2) uniform sampler2D normalBuffer;
layout(location=3) uniform sampler2D tangentBuffer;
layout(location=5) uniform sampler3D lightGrid;
layout(location=6) uniform sampler3D grid;
layout(location=7) uniform int resolution;
layout(location=8) uniform int halveCount;

out vec4 fColor;

struct Cone {
    vec3 direction;
    float cosAngle;
    float t;
};

struct TraceResult {
    vec3 colour;
};

int getGridStep(in int level, in ivec3 position) {
    ivec4 texel = ivec4(floor(texelFetch(grid, ivec3(vec3(position) / pow(2.0, level)), level) * 256.0));
    int embedded = (texel.a << 24) | (texel.b << 16) | (texel.g << 8) | (texel.r << 0);
    return (embedded & 0x000001FF) >> 0;
}

bool isSolid(in int level, in ivec3 position) {
    ivec4 texel = ivec4(floor(texelFetch(grid, ivec3(vec3(position) * pow(2.0, level)), 0) * 256.0));
    int embedded = (texel.a << 24) | (texel.b << 16) | (texel.g << 8) | (texel.r << 0);
    return 1 == ((embedded & 0x000FFE00) >> 9);
;
}

ivec2 rescalePosition(ivec2 uv) {
    const int SCALE = int(pow(2, halveCount));
    return uv * SCALE;
}

TraceResult coneTrace(in vec3 position, in Cone cone) {
    vec3 rayPos = position;
    vec3 rayDir = cone.direction;

    int level = 0;
    float radiusForNextLevel = 1.0;

    const ivec3 mapBounds = ivec3(256, 256, 256);
    const float LAMBDA = 1.0;
    const float REFLECTANCE = 1.0;

    const float inverseRadiusSqr = (1.0 / pow(cone.cosAngle, 2.0) - 1.0);

    // Sample grid and get safe cell step count via x + y + z)
    // Step ray that many cells
    // If current cell is (0, 0, 0, sdf id), then sample SDF in cell
    // Ray march via sdf when in cell with SDF

    vec3 mapPos = vec3(floor(rayPos + 0.));
    vec3 deltaDist = abs(1.0 / rayDir);
    ivec3 rayStep = ivec3(sign(rayDir));
    vec3 tMax = (sign(rayDir) * (mapPos - rayPos) + (sign(rayDir) * 0.5) + 0.5) * deltaDist;

    rayPos = floor(rayPos);
    ivec3 iMapPos = ivec3(mapPos);

    int cellId;
    int cellStep;

    TraceResult result;
    result.colour = vec3(0.0);
    const int ITER_MAX = 8;
    for (int iter = 0; iter < ITER_MAX; iter++) {
        iMapPos = ivec3(floor(mapPos));
        bool inBounds = all(lessThan(iMapPos, mapBounds)) && all(greaterThanEqual(iMapPos, ivec3(0)));
        if (!inBounds) {
            return result;
        }

        vec3 lighting = texelFetch(lightGrid, iMapPos, level).rgb;
        if (lighting != vec3(0.0)) {
            float r = sqrt(pow(cone.t, 2) * inverseRadiusSqr);
            // multiplying by pow(2, -level) because we are traversing through light voxels
            // which are not solid _but_ they do contain light data. we don't want the
            // "extra" light data, so we try to reduce the prominence
            result.colour += 1.0 / (1.0 + LAMBDA * r) * REFLECTANCE * lighting / radiusForNextLevel;
            if (isSolid(level, iMapPos)) {
                return result;
            }
        }

        int cellStep = getGridStep(level, iMapPos);

        for (int i = 0; i < cellStep; i++) {
            bvec3 mask = lessThanEqual(tMax.xyz, min(tMax.yzx, tMax.zxy));
            tMax += vec3(mask) * deltaDist;
            mapPos += vec3(mask) * rayStep;
            cone.t += dot(vec3(mask) * rayDir, vec3(1));
        }

        float r = sqrt(pow(cone.t, 2) * inverseRadiusSqr);
        if (r >= radiusForNextLevel) {
            level += 1;
            radiusForNextLevel = pow(2.0, level);
        }
    }

    return result;
}

void main() {
    if (resolution == 0) {
        return;
    }
    int index = 3 * resolution;

    vec3 position = texelFetch(positionBuffer, rescalePosition(ivec2(gl_FragCoord.xy)), 0).xyz;
    vec3 normal = texelFetch(normalBuffer, rescalePosition(ivec2(gl_FragCoord.xy)), 0).xyz;
    vec3 tangent = texelFetch(tangentBuffer, rescalePosition(ivec2(gl_FragCoord.xy)), 0).xyz;

    const float c_theta = PI / index;
    const float c_phi = 2.0 * PI / index;

    float theta = c_theta - c_theta / 2.0;
    float phi = c_phi - c_phi / 2.0;

    vec3 averageColour = vec3(0.0);
    int count = 0;

    vec3 perp = cross(tangent, normal);
    mat3 relativeMatrix = mat3(tangent, normal, perp);

    Cone cone;
    cone.t = 0.1;
    cone.cosAngle = cos(c_theta);
    for (int t = 1; t < index; t++) {
        float theta = theta * t;
        float cost = cos(theta);
        float sint = sin(theta);
        for (int p = 0; p < 2 * index; p++) {
            float phi = phi * p + (t % 2) * c_phi / 2.0;

            cone.direction = vec3(cos(phi) * sint, cost, sin(phi) * sint);
            cone.direction = relativeMatrix * cone.direction;

            TraceResult result = coneTrace(position + cone.direction * cone.t, cone);
            averageColour += result.colour;
            count += 1;
        }
    }

    cone.direction = normal;

    TraceResult result = coneTrace(position + cone.direction * cone.t, cone);
    averageColour += result.colour;
    count += 1;

    fColor = vec4(averageColour / count, 1.0);
}
