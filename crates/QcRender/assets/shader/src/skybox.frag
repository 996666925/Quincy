#version 460

layout(location = 0) in vec3 uv;
layout(location = 0) out vec4 color;
layout(binding = 0) uniform samplerCube cubemapTexture;

void main() {
    color = texture(cubemapTexture, uv);
}
