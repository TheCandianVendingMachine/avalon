#version 430

#define PI (3.1415926535)
#define INF (1.0 / 0.0)

layout(location=1) uniform usampler3D grid;
layout(location=2) uniform sampler2D albedo;
layout(location=3) uniform sampler2D tNormal;
layout(location=4) uniform sampler2D bump;
layout(location=5) uniform mat4 view;
layout(location=6) uniform mat4 inverseView;
layout(location=7) uniform mat3 projection;
layout(location=8) uniform mat3 inverseProjection;
layout(location=9) uniform vec3 cameraPos;
layout(location=10) restrict writeonly uniform image2D positionBuffer;
layout(location=11) restrict writeonly uniform image2D normalBuffer;
layout(location=12) restrict writeonly uniform image2D tangentBuffer;

in flat ivec2 screenSize;
out vec4 fColor;

void bendRay(in vec3 cameraDir, in float theta, in vec3 normal, in vec3 startRayPos, in vec3 rayPos, inout vec3 rayDir, out vec3 deltaDist, out ivec3 rayStep, out vec3 tMax) {
    vec3 mapPos = floor(rayPos);
    rayDir = cameraDir + (view * vec4(inverseProjection * (rayDir - cameraDir), 0.0)).xyz;

    float mod = 1.0 - ((dot(rayDir, normal) / cos(theta)) * 0.5 + 0.5);
    float adjustedTheta = mod * theta;

    vec3 dTick = normalize((cross(cross(rayDir, normal), rayDir)));
    vec3 newDir = cos(adjustedTheta) * rayDir + sin(adjustedTheta) * dTick;
    rayDir = newDir;

    rayDir = cameraDir + projection * (inverseView * vec4(rayDir - cameraDir, 0.0)).xyz;

    deltaDist = abs(vec3(length(rayDir)) / rayDir);
    rayStep = ivec3(sign(rayDir));
    tMax = (sign(rayDir) * (mapPos - startRayPos) + (sign(rayDir) * 0.5) + 0.5) * deltaDist;
}

float getPlaneIntersection(in vec3 normal, in vec3 center, in vec3 rayOrigin, in vec3 rayDir) {
    float denom = dot(normal, rayDir);
    if (abs(denom) > 0) {
        float t = dot(center - rayOrigin, normal) / denom;
        if (t >= 0) {
            return t;
        }
    }
    return INF;
}

vec3 rotate(vec3 p, vec3 axis, float angle) {
    axis = normalize(axis);
    vec3 proj = dot(axis, p) * axis;
    return proj + cos(angle) * (p - proj) + sin(angle) * cross(axis, p);
}

void getGridData(in ivec3 position, out bool empty, out bool opaque, out int safeStep, out int cell) {
    int texel = int(texelFetch(grid, position, 0).r);
    int collisionFlag = (texel & 0x0000001F) >> 0;
    empty = 1 == ((texel & 0x00000020) >> 5);
    opaque = 1 == ((texel & 0x00000040) >> 6);
    safeStep = (texel & 0x0000FF80) >> 7;
    cell = (texel & 0x0001FF80) >> 7;
}

void main()
{
    const ivec3 mapBounds = ivec3(32);
    vec2 uv = gl_FragCoord.xy / vec2(screenSize) * 2.0 - 1.0;

    vec3 screenPos = vec3(uv, 0.0);
    vec3 rayPos = cameraPos;
    vec3 cameraDir = (vec4(0, 0, 1, 0) * view).xyz;
    vec3 rayDir = vec3((projection * screenPos).xy, 1);
    rayDir = (view * vec4(rayDir, 0)).xyz;
    // Sample grid and get safe cell step count via x + y + z)
    // Step ray that many cells
    // If current cell is (0, 0, 0, sdf id), then sample SDF in cell
    // Ray march via sdf when in cell with SDF

    vec3 mapPos = vec3(floor(rayPos + 0.));
    vec3 deltaDist = abs(1.0 / rayDir);
    ivec3 rayStep = ivec3(sign(rayDir));
    vec3 tMax = (sign(rayDir) * (mapPos - rayPos) + (sign(rayDir) * 0.5) + 0.5) * deltaDist;
    ivec3 tMoved = ivec3(0);
    ivec3 tMovedTick = tMoved;

    rayPos = floor(rayPos);
    ivec3 iMapPos = ivec3(mapPos);

    const float LIGHT_BEND_T = 0 * PI / 180.0;
    int cellId;
    int cellStep;
    bool cellEmpty;
    bool cellOpaque;
    getGridData(iMapPos, cellEmpty, cellOpaque, cellStep, cellId);
    int previousCell = -1;
    bvec3 mask;

    vec3 normalCameraDir = normalize(vec3(0, 0, 1));
    if (!cellEmpty && cellId == 2) {
        mask = greaterThan(rayDir.xyz, max(rayDir.yzx, rayDir.zxy));
        bendRay(cameraDir, -LIGHT_BEND_T, normalCameraDir, cameraPos, mapPos, rayDir, deltaDist, rayStep, tMax);
        previousCell = cellId;
    }

    bool previousBounds = false;

    vec3 tint = vec3(1.0, 1.0, 1.0);
    int iter;
    const int ITER_MAX = int(ceil(sqrt(32 * 32 * 32)));
    for (iter = 0; iter < ITER_MAX; iter++) {
        iMapPos = ivec3(floor(mapPos));
        bool inBounds = all(lessThan(iMapPos, mapBounds)) && all(greaterThanEqual(iMapPos, ivec3(0)));
        if (previousBounds && !inBounds) { iter = ITER_MAX; break; }
        if (!previousBounds) { iter += 3; }

        if (inBounds) {
            getGridData(iMapPos, cellEmpty, cellOpaque, cellStep, cellId);

            if (!cellEmpty) {
                if (cellId == 1) {
                    break;
                } else if (cellId == 2) {
                    cellStep = 1;
                    tint = vec3(0.9, 0.7, 0.7);
                    if (previousCell != 2) {
                        // bend light
                        bendRay(cameraDir, -LIGHT_BEND_T, normalCameraDir, cameraPos, mapPos, rayDir, deltaDist, rayStep, tMax);
                        tMovedTick = tMoved;
                    }
                } else if (previousCell == 2) {
                    // bend light back
                    bendRay(cameraDir, LIGHT_BEND_T, normalCameraDir, cameraPos, mapPos, rayDir, deltaDist, rayStep, tMax);
                    vec3 normal = -vec3(rayStep) * vec3(mask);
                    vec3 backStep = -(0.5 * -rayStep * vec3(mask) + 0.5);
                    vec3 center = mapPos - backStep + vec3(not(mask)) * 0.5;
                    float t = getPlaneIntersection(normal, center, cameraPos, rayDir);

                    vec3 intersect = cameraPos + rayDir * t;
                    vec3 dist = abs(intersect - center);
                    float mod = 1.0 - dist.z;
                    fColor = vec4(vec3(mod), 1.0);

                    // get distance to next cell
                    // interpolate stretching by this distance to avoid jitter between cells
                    float distanceToNextCell = abs(fract(mapPos - cameraPos).z);
                    vec3 tPartial = tMoved + distanceToNextCell * vec3(mask) * rayStep;

                    float exponent = mod * (dot(tPartial - vec3(tMovedTick), vec3(1)) - 1.0);
                    rayDir.z *= pow(0.5, exponent);
                    deltaDist = abs(vec3(length(rayDir)) / rayDir);
                    tMax = (sign(rayDir) * (mapPos - intersect) + (sign(rayDir) * 0.5) + 0.5) * deltaDist;
                }
            }

            iter += cellStep;
            for (int i = 0; i < cellStep; i++) {
                mask = lessThanEqual(tMax.xyz, min(tMax.yzx, tMax.zxy));
                tMax += vec3(mask) * deltaDist;
                mapPos += vec3(mask) * rayStep;
                tMoved += ivec3(mask);
            }

            previousCell = cellId;
            previousBounds = inBounds;

        } else {
            mask = lessThanEqual(tMax.xyz, min(tMax.yzx, tMax.zxy));
            tMax += vec3(mask) * deltaDist;
            mapPos += ivec3(mask) * rayStep;
            tMoved += ivec3(mask);
        }
    }

    rayStep = ivec3(sign(rayDir));
    vec3 normal;
    vec3 tangent;
    if (iter >= ITER_MAX) {
        imageStore(normalBuffer, ivec2(gl_FragCoord.xy), vec4(0.0));
        imageStore(tangentBuffer, ivec2(gl_FragCoord.xy), vec4(0.0));
        imageStore(positionBuffer, ivec2(gl_FragCoord.xy), vec4(0.0));
        gl_FragDepth = 1.0;
        fColor = vec4(0.0);
        return;
    } else if (mask.x) {
        normal = -vec3(1, 0, 0) * rayStep;
    } else if (mask.y) {
        normal = -vec3(0, 1, 0) * rayStep;
    } else if (mask.z) {
        normal = -vec3(0, 0, 1) * rayStep;
    }

    vec3 backStep = -(0.5 * -rayStep * vec3(mask) + 0.5);
    vec3 center = mapPos - backStep + vec3(not(mask)) * 0.5;
    float t = getPlaneIntersection(normal, center, cameraPos, rayDir);

    // tiny offset because if we are shooting backwards, this will be position + 1; we
    // want to be near but not at p+1
    vec3 intersect = cameraPos + rayDir * (t + 0.00001 * dot(-backStep, vec3(1.0)));
    vec3 dist = abs(intersect - center);

    //iMapPos.y -= 2;

    vec2 sizeMod = vec2(1.0 / 3.0, 1.0 / 7.0);
    vec2 offset = (1.0 - sizeMod) - sizeMod * mod(iMapPos.zy, 1.0 / sizeMod);

    vec3 colourOffset = 0.5 * vec3(1.0 / 255.0);
    vec3 normalMod = vec3(1.0);
    vec3 tangentMod = vec3(1.0);
    vec3 fragNormal;
    vec3 fragTangent;
    if (mask.x) {
        fColor = texture(albedo, offset + sizeMod * dist.zy);
        fragNormal.yz = texture(tNormal, offset + sizeMod * dist.zy).yx + colourOffset.yx;
        normalMod.x = rayStep.x;
        fragTangent = vec3(fragNormal.y, 0, fragNormal.z);
        tangentMod.y = rayStep.x;
    } else if (mask.y) {
        fColor = texture(albedo, offset + sizeMod * dist.xz);
        fragNormal.xz = texture(tNormal, offset + sizeMod * dist.xz).xy + colourOffset.xy;
        normalMod.y = rayStep.y;
        fragTangent = vec3(fragNormal.z, fragNormal.x, 0);
        tangentMod.z = rayStep.y;
    } else if (mask.z) {
        fColor = texture(albedo, offset + sizeMod * dist.xy);
        fragNormal.xy = texture(tNormal, offset + sizeMod * dist.xy).xy + colourOffset.xy;
        normalMod.z = rayStep.z;
        fragTangent = vec3(0, fragNormal.x, fragNormal.y);
        tangentMod.x = -rayStep.z;
    }

    vec3 adjustedNormal = normalMod * (2.0 * fragNormal - 1.0);
    vec3 adjustedTangent = tangentMod * (2.0 * fragTangent - 1.0);
    imageStore(normalBuffer, ivec2(gl_FragCoord.xy), vec4(adjustedNormal, 1.0));
    imageStore(tangentBuffer, ivec2(gl_FragCoord.xy), vec4(adjustedTangent, 1.0));
    imageStore(positionBuffer, ivec2(gl_FragCoord.xy), vec4(intersect, 1.0));

    fColor *= vec4(tint, 1.0);

    const float near = 0.01;
    const float m = 128.0;

    const float i_dist = m / (t + m * near);
    const float i_near = 1.0 / near;
    const float i_far = 1.0 / 10000.0;
    gl_FragDepth = (i_dist - i_near) / (i_far - i_near);
}

