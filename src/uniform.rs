use glow::HasContext;
use nalgebra::{Matrix3, Matrix4, Vector2, Vector3, Vector4};

pub trait Uniform {
    fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>);
}

impl Uniform for u8 {
    fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
        unsafe {
            gl_glow.uniform_1_u32(location.as_ref(), *self as u32);
        }
    }
}

impl Uniform for f32 {
    fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
        unsafe {
            gl_glow.uniform_1_f32(location.as_ref(), *self);
        }
    }
}

impl Uniform for i32 {
    fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
        unsafe {
            gl_glow.uniform_1_i32(location.as_ref(), *self);
        }
    }
}

impl Uniform for Matrix4<f32> {
    fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
        unsafe {
            gl_glow.uniform_matrix_4_f32_slice(location.as_ref(), false, self.as_slice());
        }
    }
}

impl Uniform for three_d_asset::Matrix4<f32> {
    fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
        let slice: &[f32; 16] = self.as_ref();
        unsafe {
            gl_glow.uniform_matrix_4_f32_slice(location.as_ref(), false, slice);
        }
    }
}

impl Uniform for Matrix3<f32> {
    fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
        unsafe {
            gl_glow.uniform_matrix_3_f32_slice(location.as_ref(), false, self.as_slice());
        }
    }
}

impl Uniform for Vector2<f32> {
    fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
        unsafe {
            gl_glow.uniform_2_f32(location.as_ref(), self.x, self.y);
        }
    }
}

impl Uniform for Vector3<f32> {
    fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
        unsafe {
            gl_glow.uniform_3_f32(location.as_ref(), self.x, self.y, self.z);
        }
    }
}

impl Uniform for three_d_asset::Vector3<f32> {
    fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
        unsafe {
            gl_glow.uniform_3_f32(location.as_ref(), self.x, self.y, self.z);
        }
    }
}

impl Uniform for Vector4<f32> {
    fn set_uniform(&self, gl_glow: &glow::Context, location: Option<glow::UniformLocation>) {
        unsafe {
            gl_glow.uniform_4_f32(location.as_ref(), self.x, self.y, self.z, self.w);
        }
    }
}
