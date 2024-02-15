
#version 330 core
uniform int Mode = 0;
in vec2 texCoord;
in vec3 color;
in vec3 normal;
out vec4 FragColor;
void main()
{
   switch (Mode)
   {
   default:
   case 0:
       FragColor = vec4(1.0f, 1.0f, 1.0f, 1.0f);
       break;
   case 1:
       FragColor = vec4(fract(texCoord), 0.0f, 1.0f);
       break;
   case 2:
       FragColor = vec4(color, 1.0f);
       break;
   case 3:
       FragColor = vec4(normalize(normal), 1.0f);
       break;
   case 4:
       FragColor = vec4(color * max(dot(normalize(normal), normalize(vec3(1,0,1))), 0.2f), 1.0f);
       break;
   }
}