#version 330
in vec3 position;
in vec3 vertColor;
out vec3 color;
uniform float translX;
void main() {
    gl_Position = vec4 (position.x+translX,position.y,position.z,1.0);
    color = vertColor;
}