#version 150 core

uniform sampler2D tex;

in vec2 v_Uv;
in vec4 v_Color;

out vec4 o_Color;

void main() {
    o_Color = texture(tex, v_Uv) * v_Color;
}
