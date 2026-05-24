#version 330

in vec2 fragTexCoord;
in vec4 fragColor;

uniform sampler2D texture0;
uniform vec4 colDiffuse;

out vec4 finalColor;

uniform vec2 renderResolution;

void main() {
    vec2 texelSize = 1.0 / renderResolution;
    vec4 sum = vec4(0.0);
    
    for (int x = -1; x <= 1; x++) {
        for (int y = -1; y <= 1; y++) {
            vec2 offset = vec2(float(x), float(y)) * texelSize * 2.0;
            sum += texture2D(texture0, fragTexCoord + offset);
        }
    }
    
    finalColor = (sum / 9.0) * vec4(0.7, 0.7, 0.7, 1.0);
}