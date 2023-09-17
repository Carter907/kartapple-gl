use crate::core::uniform::Uniform;
use cgmath::*;
use gl::types::*;
use std::f32::consts::PI;
/// Camera for projecting a 3d scene unto a 2d screen. provides many methods for maniputing the view, model, and projection matrices.

pub struct Camera {
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
    model: Matrix4<f32>,
    model_id: GLint,
    view_id: GLint,
    proj_id: GLint,
}

impl Camera {
    /// Creates a new Camera. takes in the program of the app as well as the model, view, and projection matrices.
    pub unsafe fn new(
        program: GLuint,
        model_name: &str,
        view_name: &str,
        projection_name: &str,
    ) -> Camera {
        Camera {
            projection: Matrix4::identity(),
            view: Matrix4::identity(),
            model: Matrix4::identity(),
            model_id: Uniform::locate_uniform(program, model_name),
            view_id: Uniform::locate_uniform(program, view_name),
            proj_id: Uniform::locate_uniform(program, projection_name),
        }
    }
    /// Should be called in the on_loop method, this updates the Camera matrices.
    pub unsafe fn update(&self) {
        gl::UniformMatrix4fv(self.model_id, 1, gl::FALSE, self.model.as_ptr());
        gl::UniformMatrix4fv(self.view_id, 1, gl::FALSE, self.view.as_ptr());
        gl::UniformMatrix4fv(self.proj_id, 1, gl::FALSE, self.projection.as_ptr());
    }
    /// initializes the projection matrix given a cgmath::projection::PerspectiveFov
    pub fn projection(&mut self, perspective: PerspectiveFov<f32>) {
        self.projection = Matrix4::from(perspective);
    }
    /// rotates the view by angle in the y axis. angle is measured in degrees.
    pub fn rotate_view_y(&mut self, angle: f32) {
        self.view = self.view * Matrix4::from_angle_y(Deg(angle));
    }

    pub fn rotate_view_x(&mut self, angle: f32) {
        self.view = self.view * Matrix4::from_angle_x(Deg(angle));
    }

    pub fn rotate_view_z(&mut self, angle: f32) {
        self.view = self.view * Matrix4::from_angle_z(Deg(angle));
    }
    pub fn rotate_model_y(&mut self, angle: f32) {
        self.model = self.model * Matrix4::from_angle_y(Deg(angle))
    }
    pub fn rotate_model_x(&mut self, angle: f32) {
        self.model = self.model * Matrix4::from_angle_x(Deg(angle));
    }

    pub fn translate_view(&mut self, vec: Vector3<f32>) {
        self.view = self.view * Matrix4::from_translation(vec);
    }
    pub fn translate_view_z(&mut self, angle: f32) {
        self.view = self.view * Matrix4::from_translation(Vector3::new(0f32, 0f32, angle));
    }
    pub fn translate_view_y(&mut self, angle: f32) {
        self.view = self.view * Matrix4::from_translation(Vector3::new(0f32, angle, 0f32));
    }
    pub fn translate_view_x(&mut self, angle: f32) {
        self.view = self.view * Matrix4::from_translation(Vector3::new(angle, 0f32, 0f32));
    }

}
