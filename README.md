# gl-shader-player

A simple program that reads and executes shader programs.

## Usage

1. Copy `config.example.yml` to `config.yml`.
2. Put your vertex and fragment shaders into `shaders` folder. The shaders must have a same filename and have `.vert` or `.frag` as their extension.
3. Edit your `config.yml` following comments in it.
4. Run `cargo run` or build and run the program by yourself.

## Note

Please do not include non-ascii chars, like CJK characters or BOM bytes, in your shader files. Otherwise you'll encounter compilation error.

## Input

The program only provide a `vec2 position` as input for the vertex shader, representing coordinates of four vertices located on the upper left (-1, 1), upper right (1, 1), lower right (1, -1), and lower left (-1, -1).

## Uniforms

The program currently provide two shader uniforms.

- `vec3 iResolution`: Represents the resolution of the render area. `iResolution.x` represents its width, and `iResolution.y` represents its height. `iResolution.z` is reserved for compatibility, and will always be `0.0`.
- `float iTime`: Elapsed time since the program started running, in seconds. This is widely used to generate animated visual effects.
