use bevy::prelude::{Camera, GlobalTransform, Vec2, Window};

pub fn screen_to_world(screen_pos: Vec2, screen_size: Vec2,
                       camera: &Camera, camera_transform: &GlobalTransform)
                       -> Vec2 {
    let ndc = screen_pos_to_ndc(screen_pos, screen_size);

    ndc_to_world(ndc, camera, camera_transform)
}

pub fn world_to_screen(world_pos: Vec2, window_height: f32,
                       camera: &Camera, camera_transform: &GlobalTransform)
                       -> Vec2 {
    let pos = camera.world_to_viewport(camera_transform, world_pos.extend(0.))
        .unwrap_or(Vec2::ZERO);

    pos * Vec2::new(1., -1.) + Vec2::Y * window_height
}

pub fn camera_world_bounds(screen_size: Vec2, camera: &Camera,
                           camera_transform: &GlobalTransform) -> [Vec2; 2] {
    let screen_to_world = |screen_pos: Vec2| {
        screen_to_world(screen_pos, screen_size, camera, camera_transform)
    };

    let cam_min: Vec2 = screen_to_world(Vec2::ZERO);
    let cam_max: Vec2 = screen_to_world(screen_size);

    [cam_min, cam_max]
}

/// convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
pub fn screen_pos_to_ndc(screen_pos: Vec2, screen_size: Vec2) -> Vec2 {
    (screen_pos / screen_size) * 2.0 - Vec2::ONE
}

pub fn ndc_to_world(ndc: Vec2, camera: &Camera, camera_trans: &GlobalTransform) -> Vec2 {
    // matrix for undoing the projection and camera transform
    let conversion_mat = camera_trans.compute_matrix() * camera.projection_matrix().inverse();

    // use it to convert ndc to world-space coordinates
    conversion_mat.project_point3(ndc.extend(-1.0)).truncate()
}

pub trait ScreenSize {
    fn screen_size(&self) -> Vec2;
}

impl ScreenSize for Window {
    fn screen_size(&self) -> Vec2 {
        let size = Vec2::new(self.width(), self.height());
        if size.x == 0. || size.y == 0. {
            Vec2::ZERO
        } else {
            size
        }
    }
}