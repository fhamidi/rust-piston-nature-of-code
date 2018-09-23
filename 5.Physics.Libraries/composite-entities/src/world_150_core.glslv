#version 150 core

in vec2 pos;
in vec2 uv;
in vec4 color;

uniform vec4 transform;

out vec2 v_Uv;
out vec4 v_Color;

void main() {
    vec2 position = vec2(
        pos.x * transform.z + transform.x,
        pos.y * transform.w + transform.y);
    gl_Position = vec4(position, 0.0, 1.0);
    v_Uv = uv;
    v_Color = color;
}
