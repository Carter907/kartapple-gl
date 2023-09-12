#version 330
in vec3 pos;
in vec3 vertColor;
out vec3 color;
uniform mat4 model;
uniform mat4 proj;
uniform mat4 view;

void main() {

    gl_Position = proj * view * model * vec4(pos, 1.0);

    color = vertColor;
}
