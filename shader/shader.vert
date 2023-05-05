#version 330 core
layout (location = 0) in vec3 position;

uniform mat4 matrix;
out vec4 color;

void main() {
    color = matrix * vec4(position, 1.0);
    gl_Position = matrix * vec4(position.x, position.y, position.z, 1.0);
}