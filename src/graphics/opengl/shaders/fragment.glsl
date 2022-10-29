#version 330 core

in vec3 Colour;
out vec4 FragColor;

void main() {
    FragColor = vec4(Colour, 1.0);
}