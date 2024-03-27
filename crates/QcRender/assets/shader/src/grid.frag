#version 460

layout(location = 0) out vec4 color;

layout(binding = 6) uniform MVP {
    mat4 model;
    mat4 view;
    mat4 proj;
    vec3 view_pos;
};

layout(location = 0) in vec3 fragPos;
layout(location = 1) in vec2 uv;

layout(location = 0) uniform vec3 uColor;

float MAG(float p_lp) {
    const float lineWidth = 1.0;

    const vec2 coord = uv / p_lp;
    const vec2 grid = abs(fract(coord - 0.5) - 0.5) / fwidth(coord);
    const float line = min(grid.x, grid.y);
    const float lineResult = lineWidth - min(line, lineWidth);

    return lineResult;
}

float Grid(float height, float a, float b, float c) {
    const float cl = MAG(a);
    const float ml = MAG(b);
    const float fl = MAG(c);

    const float cmit = 10.0;
    const float cmet = 40.0;
    const float mfit = 80.0;
    const float mfet = 160.0;

    const float df = clamp((height - cmit) / (cmet - cmit), 0.0, 1.0);
    const float dff = clamp((height - mfit) / (mfet - mfit), 0.0, 1.0);

    const float inl = mix(cl, ml, df);
    const float fnl = mix(inl, fl, dff);

    return fnl;
}

void main() {
    const float height = distance(view_pos.y, fragPos.y);

    const float gridA = Grid(height, 1.0, 4.0, 8.0);
    const float gridB = Grid(height, 4.0, 16.0, 32.0);

    const float grid = gridA * 0.5 + gridB;

    const vec2 viewdirW = view_pos.xz - fragPos.xz;
    const float viewdist = length(viewdirW);

    color = vec4(uColor, grid);
}