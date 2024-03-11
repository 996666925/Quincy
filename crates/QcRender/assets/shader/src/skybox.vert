#version 460 

layout(location = 0) in vec3 position;
layout(location = 1) in vec2 texCoords;
layout(location = 2) in vec3 normal;
layout(location = 0) out vec3 uv;
layout(binding = 6) uniform MVP {
    mat4 model;
    mat4 view;
    mat4 proj;
};

void main() {
    gl_Position = proj * view * model * vec4(position, 1.0);
    uv = position;
}