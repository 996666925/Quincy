#version 460

layout(location = 0) in vec3 aColor;

layout(location = 0) out vec4 color;


void main() {
    color = vec4(aColor, 1.0);
}