#version 460

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 color;
layout(binding = 0) uniform sampler2D uDiffuseMap;

void main() {
    color = texture(uDiffuseMap, uv);
}
