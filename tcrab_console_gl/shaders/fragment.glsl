#version 330

in vec2 v_TexCoord;
in vec3 v_ForegroundColor;
in vec3 v_BackgroundColor;

out vec4 o_Color;

uniform sampler2D u_Texture;

void main() {
    float alpha = texture2D(u_Texture, v_TexCoord).a;
    o_Color = vec4(mix(v_BackgroundColor, v_ForegroundColor, vec3(alpha)), 1.0);
}