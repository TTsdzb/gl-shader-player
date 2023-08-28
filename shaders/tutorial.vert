#version 330 core

in vec2 position;
  
out vec2 pos;

uniform vec3 iResolution;

void main()
{
  gl_Position = vec4(position, 1., 1.);

  if (iResolution.x >= iResolution.y) pos = vec2(position.x / iResolution.y * iResolution.x, position.y);
  else pos = vec2(position.x, position.y / iResolution.x * iResolution.y);
}
