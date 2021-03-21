
extern crate gl;

use std::fs;
use std::path::PathBuf;
use std::ffi::CString;

use crate::te::paths;
use crate::gl_wrapper::Shader;
use crate::gl_wrapper::gl_info::GLInfo;

use crate::utils::regex::*;
use std::collections::HashMap;
use crate::te::paths::PathBufExt;

pub struct ShaderCompiler {
    pub gl_info: GLInfo
}

impl ShaderCompiler {

    fn version(&self) -> String {
        let mut result = "#version ".to_string();
        result += &self.gl_info.glsl_version;
        if self.gl_info.is_gles {
            result += " es";
        }
        else {
            result += " core";
        }
        result + "\n"
    }

    fn check_programm_error(path: PathBuf, program: u32) {
        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(program, gl::COMPILE_STATUS, &mut success);
        }
        if success != 0 { return; }
        let mut len: gl::types::GLint = 0;

        unsafe {
            gl::GetShaderiv(program, gl::INFO_LOG_LENGTH, &mut len);
        }

        fn alloc_str(len: usize) -> CString {
            let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
            buffer.extend([b' '].iter().cycle().take(len));
            unsafe { CString::from_vec_unchecked(buffer) }
        }

        let error = alloc_str(len as usize);

        unsafe {
            gl::GetShaderInfoLog(program, len, std::ptr::null_mut(), error.as_ptr() as *mut gl::types::GLchar);
        }

        let error = error.to_string_lossy().into_owned();
        panic!("Failed to compile shader: {:?} error: {}", path, error);
    }

    fn unfold_includes(mut code: String) -> String {
        const QUOTES_QUERY: &'static str = r#"(("[^ "]+"))"#;
        const INCLUDE_QUERY: &'static str = r#"#include (("[^ "]+"))"#;
        let includes = find_matches(&code, INCLUDE_QUERY);
        let mut files: HashMap<String, String> = HashMap::new();
        for include in includes {
            let file_name = find_match(&include, QUOTES_QUERY).replace("\"", "");
            let file_path = paths::shaders::include().pushing(file_name);
            let include_code = fs::read_to_string(file_path).unwrap();
            files.insert(include, include_code);
        }
        for (include, include_code) in files {
            code = code.replace(include.as_str(), include_code.as_str());
        }
        code
    }

    fn compile_shader(&self, path: PathBuf, code: String, kind: gl::types::GLenum) -> u32 {

        let code = self.version() + "\n" + &ShaderCompiler::unfold_includes(code);

        println!("{}", code);

        let shader = unsafe { gl::CreateShader(kind) };

        unsafe {
            let c_code = CString::new(code.clone()).unwrap();
            let code_ptr = c_code.as_ptr();
            gl::ShaderSource(shader, 1, &code_ptr, std::ptr::null());
            gl::CompileShader(shader);
        }

        ShaderCompiler::check_programm_error(path, shader);

        shader
    }

    pub fn compile(&self, path: PathBuf) -> Shader {

        let vert_path = path.with_extension("vert");
        let frag_path = path.with_extension("frag");

        let vert_code = fs::read_to_string(&vert_path).unwrap();
        let frag_code = fs::read_to_string(&frag_path).unwrap();

        let vert = self.compile_shader(vert_path, vert_code, gl::VERTEX_SHADER);
        let frag = self.compile_shader(frag_path, frag_code, gl::FRAGMENT_SHADER);

        let program = unsafe { gl::CreateProgram() };

        unsafe {
            gl::AttachShader(program, vert);
            gl::AttachShader(program, frag);
            gl::LinkProgram(program);
        }

        ShaderCompiler::check_programm_error(path, program);

        unsafe  {
            gl::DetachShader(program, vert);
            gl::DetachShader(program, frag);

            gl::DeleteShader(vert);
            gl::DeleteShader(frag);
        }

        Shader::new(program)
    }

}
