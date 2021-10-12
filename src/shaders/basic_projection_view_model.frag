#version 330
in vec2 v_tex_coords;

out vec4 color;
uniform sampler2D diffuse_tex;

void main() {
    vec4 diffuse_color = texture(diffuse_tex, v_tex_coords) ;
        if(diffuse_color.a < 0.1)
            discard;
        color = vec4(diffuse_color);
}