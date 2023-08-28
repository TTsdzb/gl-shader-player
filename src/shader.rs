use std::fs;
use thiserror::Error;


/// Error that can be triggered when `load_shader`.
#[derive(Error, Debug)]
pub enum ShaderLoadError {
    #[error("Error occurred while reading shader: {0}")]
    ReadFailed(
        #[from]
        #[source]
        std::io::Error,
    ),
    #[error("Error occurred while compiling shader programs: {0}")]
    CompileFailed(
        #[from]
        #[source]
        glium::program::ProgramCreationError,
    ),
}

/// Load vertex and fragment shader with given display.
///
/// Currently paths are not supported, and `name` is used to specify files should be loaded.
///
/// For example, if a name `tutorial` is given, the function will try to find and load `./shaders/tutorial.vert` and `./shaders/tutorial.frag`.
/// 
/// # Example
/// 
/// ```rs
/// let program = shader::load_shader(&display, &conf.shader)?;
/// ```
pub fn load_shader(
    display: &glium::backend::glutin::Display,
    name: &str,
) -> Result<glium::Program, ShaderLoadError> {
    let vertex_shader_path = format!("shaders/{name}.vert");
    let vertex_shader_src = fs::read_to_string(&vertex_shader_path)?;

    let fragment_shader_path = format!("shaders/{name}.frag");
    let fragment_shader_src = fs::read_to_string(&fragment_shader_path)?;

    Ok(glium::Program::from_source(
        display,
        &vertex_shader_src,
        &fragment_shader_src,
        None,
    )?)
}
