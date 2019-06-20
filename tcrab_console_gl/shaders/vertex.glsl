#version 330

layout(location = 0) in vec2 a_Position;
layout(location = 1) in vec2 a_TexCoord;
layout(location = 2) in vec3 a_ForegroundColor;
layout(location = 3) in vec3 a_BackgroundColor;

out vec2 v_TexCoord;
out vec3 v_ForegroundColor;
out vec3 v_BackgroundColor;

uniform mat4 u_Projection;

void main() {
    gl_Position = u_Projection * vec4(a_Position, 0.0, 1.0);
    v_TexCoord = a_TexCoord;
    v_ForegroundColor = a_ForegroundColor;
    v_BackgroundColor = a_BackgroundColor;
}