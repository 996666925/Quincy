#version 460

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 color;
layout(binding = 0) uniform sampler2D uDiffuseMap;
layout(location = 5) uniform vec4 uDiffuse = vec4(1.0, 1.0, 1.0, 1.0);

void main() {
    color = texture(uDiffuseMap, uv) * uDiffuse;
}
