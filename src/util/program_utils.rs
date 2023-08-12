use crate::util::shader_utils::ShaderUtils;

pub struct ProgramUtils {}

impl ProgramUtils {
    pub unsafe fn create_program(vert_code: &String, frag_code: &String) -> gl::types::GLuint {
        let program_id = gl::CreateProgram();

        let vert_id = ShaderUtils::get_shader(gl::VERTEX_SHADER, vert_code);
        let frag_id = ShaderUtils::get_shader(gl::FRAGMENT_SHADER, frag_code);
        gl::AttachShader(program_id, vert_id);
        gl::AttachShader(program_id, frag_id);
        gl::LinkProgram(program_id);
        gl::DeleteShader(vert_id);
        gl::DeleteShader(frag_id);
        program_id
    }
}