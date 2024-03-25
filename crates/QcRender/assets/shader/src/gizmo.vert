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

layout(location = 0) out vec3 color;

layout(location = 0) uniform bool uIsBall;
layout(location = 1) uniform bool uIsPickable;
layout(location = 2) uniform int uHighlightedAxis = 3;

mat4 rotationMatrix(vec3 axis, float angle) {
    axis = normalize(axis);
    float s = sin(angle);
    float c = cos(angle);
    float oc = 1.0 - c;

    return mat4(oc * axis.x * axis.x + c, oc * axis.x * axis.y - axis.z * s, oc * axis.z * axis.x + axis.y * s, 0.0, oc * axis.x * axis.y + axis.z * s, oc * axis.y * axis.y + c, oc * axis.y * axis.z - axis.x * s, 0.0, oc * axis.z * axis.x - axis.y * s, oc * axis.y * axis.z + axis.x * s, oc * axis.z * axis.z + c, 0.0, 0.0, 0.0, 0.0, 1.0);
}

void main() {
    mat4 instanceModel = model;

    if(gl_InstanceID == 1)
        instanceModel *= rotationMatrix(vec3(0, 1, 0), radians(-90)); /* X axis */
    else if(gl_InstanceID == 2)
        instanceModel *= rotationMatrix(vec3(1, 0, 0), radians(90)); /* Y axis */

    float distanceToCamera = distance(view_pos, instanceModel[3].xyz);

    vec3 pos = position;

    vec3 fragPos = vec3(instanceModel * vec4(pos * distanceToCamera * 0.1, 1.0));

    if(uIsPickable) {
        int blueComponent = 0;

        if(gl_InstanceID == 1)
            blueComponent = 252;

        if(gl_InstanceID == 2)
            blueComponent = 253;

        if(gl_InstanceID == 0)
            blueComponent = 254;

        color = vec3(1.0, 1.0, blueComponent / 255.0);
    } else {
        if(uIsBall) {
            color = vec3(1.0);
        } else {
            float red = float(gl_InstanceID == 1); // X
            float green = float(gl_InstanceID == 2); // Y
            float blue = float(gl_InstanceID == 0); // Z

            if(!uIsPickable && ((gl_InstanceID == 1 && uHighlightedAxis == 0) || (gl_InstanceID == 2 && uHighlightedAxis == 1) || (gl_InstanceID == 0 && uHighlightedAxis == 2))) {
                color = vec3(1.0, 1.0, 0.0);
            } else {
                color = vec3(red, green, blue);
            }
        }
    }

    gl_Position = proj * view * vec4(fragPos, 1.0);
}