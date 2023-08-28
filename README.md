# gl-shader-player

A simple program that reads and executes shader programs.

## Usage

1. Copy `config.example.yml` to `config.yml`.
2. Put your vertex and fragment shaders into `shaders` folder. The shaders must have a same filename and have `.vert` or `.frag` as their extension.
3. Edit your `config.yml` following comments in it.
4. Run `cargo run` or build and run the program by yourself.

## Note

Please do not include non-ascii chars, like CJK characters or BOM bytes, in your shader files. Otherwise you'll encounter compilation error.
