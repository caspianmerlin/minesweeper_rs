#version 330 core

layout (location = 0) in vec2 inPosition;
layout (location = 1) in vec3 inColour;

out vec3 Colour;

void main() {
    Colour = inColour;
    gl_Position = vec4(inPosition, 0.0, 1.0);
}