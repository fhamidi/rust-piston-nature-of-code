#version 150 core

in vec2 pos;
in vec2 uv;
in vec4 color;

out vec2 v_uv;
out vec4 v_color;

void main() {
    gl_Position = vec4(pos, 0.0, 1.0);
    v_uv = uv;
    v_color = color;
}
