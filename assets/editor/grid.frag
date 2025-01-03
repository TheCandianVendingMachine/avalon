#version 430

layout(location = 1) uniform sampler3D grid;
layout(location = 2) uniform sampler2D albedo;
layout(location = 3) uniform ivec3 direction;
layout(location = 4) uniform vec2 position;
layout(location = 5) uniform float zoom;
layout(location = 6) uniform ivec3 highlightedCell;

in vec2 screenSize;
out vec4 fColor;

void getGridData(in ivec3 position, out int safeStep, out int cell) {
    ivec4 texel = ivec4(floor(texelFetch(grid, position, 0) * 256.0));
    int embedded = (texel.a << 24) | (texel.b << 16) | (texel.g << 8) | (texel.r << 0);
    safeStep = (embedded & 0x000001FF) >> 0;
    cell = (embedded & 0x000FFE00) >> 9;
    int unused = (embedded & 0xFFF00000) >> 20;
}

void main() {
    vec2 ratio = vec2(32, 18);
    vec2 chunkSize = vec2(16, 16);

    vec2 uv = position + zoom * (ratio * gl_FragCoord.xy / screenSize - 0.5 * ratio) + 0.5 * ratio;
    ivec2 index = ivec2(floor(uv));
    uv = fract(uv);

    // dont render outside the grid
    if (any(lessThan(index, ivec2(0))) || any(greaterThanEqual(index, ivec2(256)))) {
        fColor = vec4(1.0);
        return;
    }

    // render any voxels
    ivec3 gridSize = ivec3(textureSize(grid, 0));
    if (direction.x != 0) {
        for (int i = 0; i < gridSize.x; i++) {
            ivec3 samplePos = ivec3(gridSize.x - i - 1, index.y, index.x);
            int safeStep;
            int cell;
            getGridData(samplePos, safeStep, cell);
            if (cell != 0) {
                fColor = vec4(0.9, 0.1, 0.7, 1.0);
                return;
            }
        }
    } else if (direction.y != 0) {
        for (int i = 0; i < gridSize.y; i++) {
            ivec3 samplePos = ivec3(index.x, gridSize.y - i - 1, index.y);
            int safeStep;
            int cell;
            getGridData(samplePos, safeStep, cell);
            if (cell != 0) {
                fColor = vec4(0.9, 0.1, 0.7, 1.0);
                return;
            }
        }
    } else if (direction.z != 0) {
        for (int i = 0; i < gridSize.z; i++) {
            ivec3 samplePos = ivec3(index.x, index.y, gridSize.z - i - 1);
            int safeStep;
            int cell;
            getGridData(samplePos, safeStep, cell);
            if (cell != 0) {
                fColor = vec4(0.9, 0.1, 0.7, 1.0);
                return;
            }
        }
    }

    float pixels = 2 / (1 + exp(zoom));
    vec2 lineWidth = (1.0 - fwidth(uv) * max(pixels, 1));
    vec2 intersectWidth = (1.0 - fwidth(uv) * max(5 / (zoom + 1), 1));

    bvec2 bChunkColourChange = equal(floor(mod(index, chunkSize) / (chunkSize - 1)), vec2(1));
    vec3 chunkColourChange = vec3(any(bChunkColourChange));

    if ((any(greaterThan(uv, intersectWidth)) && any(lessThan(uv, 1.0 - intersectWidth))) ||
        (all(greaterThan(uv, intersectWidth)) || all(lessThan(uv, 1.0 - intersectWidth)))) {
        // intersection squares
        fColor = vec4(vec3(0.4), 1);
    } else if (any(greaterThan(uv, lineWidth))) {
        fColor = vec4(vec3(0.4), 1);
        if (bChunkColourChange.x && uv.x > lineWidth.x) {
            fColor *= vec4(1.5, 0.2, 0.2, 1);
        }
        if (bChunkColourChange.y && uv.y > lineWidth.y) {
            fColor *= vec4(1.5, 0.2, 0.2, 1);
        }
    } else {
        fColor = vec4(vec3(0.14, 0.14, 0.32), 0);
    }
}
