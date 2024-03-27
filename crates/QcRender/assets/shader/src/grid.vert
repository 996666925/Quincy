#version 460

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 texCoords;
layout(location = 2) in vec3 normal;

layout(binding = 6) uniform MVP {
    mat4 model;
    mat4 view;
    mat4 proj;
    vec3 view_pos;
};

layout(location = 0) out vec3 fragPos;
layout(location = 1) out vec2 uv;


void main() {
    fragPos = vec3(model * vec4(position, 1.0));
    uv = fragPos.xz;

    gl_Position = proj * view * vec4(fragPos, 1.0);
}
