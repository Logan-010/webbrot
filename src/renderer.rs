use crate::{options::Options, prepare_view};
use std::cell::Cell;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, WebGl2RenderingContext as Gl, WebGlProgram, WebGlShader, WebGlTexture,
    WebGlUniformLocation, WebGlVertexArrayObject,
};

/// Reuses the shader program and palette texture across generations. Pixels never leave the GPU.
pub struct Renderer {
    canvas: HtmlCanvasElement,
    gl: Gl,
    program: WebGlProgram,
    texture: WebGlTexture,
    vao: WebGlVertexArrayObject,
    bounds: WebGlUniformLocation,
    dimensions: WebGlUniformLocation,
    max_steps: WebGlUniformLocation,
    bailout: WebGlUniformLocation,
    has_image: Cell<bool>,
}

impl Renderer {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, String> {
        let gl: Gl = canvas
            .get_context("webgl2")
            .map_err(|e| format!("Cannot create a WebGL2 context: {e:?}"))?
            .ok_or("WebGL2 is unavailable. Enable hardware acceleration or try another browser.")?
            .dyn_into()
            .map_err(|_| "Unexpected canvas context type.")?;
        if gl.is_context_lost() {
            return Err("The GPU context is still lost. Wait for it to recover.".into());
        }
        let vertex = compile_shader(&gl, Gl::VERTEX_SHADER, include_str!("mandelbrot.vert"))?;
        let fragment =
            match compile_shader(&gl, Gl::FRAGMENT_SHADER, include_str!("mandelbrot.frag")) {
                Ok(shader) => shader,
                Err(error) => {
                    gl.delete_shader(Some(&vertex));
                    return Err(error);
                }
            };
        let program = gl
            .create_program()
            .ok_or("Cannot allocate a shader program.")?;
        gl.attach_shader(&program, &vertex);
        gl.attach_shader(&program, &fragment);
        gl.link_program(&program);
        gl.delete_shader(Some(&vertex));
        gl.delete_shader(Some(&fragment));
        if !gl
            .get_program_parameter(&program, Gl::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            let error = gl.get_program_info_log(&program).unwrap_or_default();
            gl.delete_program(Some(&program));
            return Err(format!("Shader linking failed: {error}"));
        }
        let uniform = |name| {
            gl.get_uniform_location(&program, name)
                .ok_or_else(|| format!("Missing shader uniform: {name}"))
        };
        let bounds = uniform("bounds")?;
        let dimensions = uniform("dimensions")?;
        let max_steps = uniform("max_steps")?;
        let bailout = uniform("bailout")?;
        let texture = gl
            .create_texture()
            .ok_or("Cannot allocate a palette texture.")?;
        let vao = gl
            .create_vertex_array()
            .ok_or("Cannot allocate a vertex array.")?;
        gl.use_program(Some(&program));
        gl.uniform1i(Some(&uniform("palette")?), 0);
        gl.active_texture(Gl::TEXTURE0);
        gl.bind_texture(Gl::TEXTURE_2D, Some(&texture));
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_MIN_FILTER, Gl::NEAREST as i32);
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_MAG_FILTER, Gl::NEAREST as i32);
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_WRAP_S, Gl::CLAMP_TO_EDGE as i32);
        gl.tex_parameteri(Gl::TEXTURE_2D, Gl::TEXTURE_WRAP_T, Gl::CLAMP_TO_EDGE as i32);
        gl.pixel_storei(Gl::UNPACK_ALIGNMENT, 1);
        gl.disable(Gl::DITHER);
        Ok(Self {
            canvas,
            gl,
            program,
            texture,
            vao,
            bounds,
            dimensions,
            max_steps,
            bailout,
            has_image: Cell::new(false),
        })
    }

    pub fn is_context_lost(&self) -> bool {
        self.gl.is_context_lost()
    }

    pub fn has_image(&self) -> bool {
        self.has_image.get() && !self.is_context_lost()
    }

    /// Redraw the last frame without rerunning view selection. WebGL may discard its
    /// drawing buffer after presentation, so exporting must start in this same task.
    pub fn redraw(&self) -> Result<(), String> {
        if !self.has_image() {
            return Err("Generate an image before saving.".into());
        }
        let gl = &self.gl;
        gl.viewport(
            0,
            0,
            self.canvas.width() as i32,
            self.canvas.height() as i32,
        );
        gl.use_program(Some(&self.program));
        gl.bind_vertex_array(Some(&self.vao));
        gl.active_texture(Gl::TEXTURE0);
        gl.bind_texture(Gl::TEXTURE_2D, Some(&self.texture));
        gl.draw_arrays(Gl::TRIANGLES, 0, 3);
        let error = gl.get_error();
        if error != Gl::NO_ERROR {
            return Err(format!("GPU redraw failed (WebGL error 0x{error:x})."));
        }
        Ok(())
    }

    pub fn render(&self, options: &Options) -> Result<(), String> {
        let gl = &self.gl;
        if gl.is_context_lost() {
            return Err("The GPU context was lost. Try generating again after it recovers.".into());
        }
        let limit = gl
            .get_parameter(Gl::MAX_VIEWPORT_DIMS)
            .map_err(|e| format!("Cannot query GPU limits: {e:?}"))?;
        let limits = js_limits(limit)?;
        if options.dimensions[0] > limits[0] || options.dimensions[1] > limits[1] {
            return Err(format!(
                "This GPU supports dimensions up to {} × {}.",
                limits[0], limits[1]
            ));
        }
        if options.step_limits[1] > i32::MAX as u32 / 3 {
            return Err("The maximum step count is too large for the shader.".into());
        }
        let view = prepare_view(options)?;
        let bounds = view.bounds.map(|n| n as f32);
        if bounds.iter().any(|n| !n.is_finite()) {
            return Err("The view is outside the GPU's floating-point range.".into());
        }
        if bounds[0] == bounds[1] || bounds[2] == bounds[3] {
            return Err(
                "This zoom is too deep for 32-bit GPU precision. Increase the view size.".into(),
            );
        }
        let [width, height] = options.dimensions;
        self.has_image.set(false);
        self.canvas.set_width(width);
        self.canvas.set_height(height);
        if gl.drawing_buffer_width() != width as i32 || gl.drawing_buffer_height() != height as i32
        {
            return Err(
                "The GPU cannot allocate a canvas this large. Reduce the dimensions.".into(),
            );
        }
        gl.viewport(0, 0, width as i32, height as i32);
        gl.use_program(Some(&self.program));
        gl.bind_vertex_array(Some(&self.vao));
        gl.active_texture(Gl::TEXTURE0);
        gl.bind_texture(Gl::TEXTURE_2D, Some(&self.texture));
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            Gl::TEXTURE_2D,
            0,
            Gl::RGB8 as i32,
            (view.palette.len() / 3) as i32,
            1,
            0,
            Gl::RGB,
            Gl::UNSIGNED_BYTE,
            Some(view.palette),
        )
        .map_err(|e| format!("Cannot upload the colormap: {e:?}"))?;
        gl.uniform4fv_with_f32_array(Some(&self.bounds), &bounds);
        gl.uniform2f(Some(&self.dimensions), width as f32, height as f32);
        gl.uniform1i(Some(&self.max_steps), view.config.max_steps as i32);
        gl.uniform1f(Some(&self.bailout), view.config.bailout_num as f32);
        gl.draw_arrays(Gl::TRIANGLES, 0, 3);
        let error = gl.get_error();
        if error != Gl::NO_ERROR {
            return Err(format!("GPU rendering failed (WebGL error 0x{error:x})."));
        }
        self.has_image.set(true);
        Ok(())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.gl.delete_texture(Some(&self.texture));
        self.gl.delete_vertex_array(Some(&self.vao));
        self.gl.delete_program(Some(&self.program));
    }
}

fn compile_shader(gl: &Gl, kind: u32, source: &str) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(kind).ok_or("Cannot allocate a shader.")?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);
    if gl
        .get_shader_parameter(&shader, Gl::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let error = gl.get_shader_info_log(&shader).unwrap_or_default();
        gl.delete_shader(Some(&shader));
        Err(format!("Shader compilation failed: {error}"))
    }
}

fn js_limits(value: wasm_bindgen::JsValue) -> Result<[u32; 2], String> {
    // MAX_VIEWPORT_DIMS is an Int32Array, not a JavaScript Array.
    let limits = js_sys::Int32Array::new(&value);
    if limits.length() != 2 {
        return Err("Unexpected GPU viewport limits.".into());
    }
    Ok([limits.get_index(0) as u32, limits.get_index(1) as u32])
}
