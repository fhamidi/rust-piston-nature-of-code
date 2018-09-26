#version 150 core

uniform sampler2D sampler;

in vec2 v_uv;
in vec4 v_color;

out vec4 o_color;

void main() {
    o_color = texture(sampler, v_uv) * v_color;
}
