use bevy::prelude::{Camera, GlobalTransform, Vec2, Window};

pub fn screen_to_world(screen_pos: Vec2, screen_size: Vec2,
                       camera: &Camera, camera_transform: &GlobalTransform)
                       -> Vec2 {
    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (screen_pos / screen_size) * 2.0 - Vec2::ONE;

    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    world_pos.truncate()
}

pub fn world_to_screen(world_pos: Vec2, window_height: f32,
                       camera: &Camera, camera_transform: &GlobalTransform)
                       -> Vec2 {
    let pos = camera.world_to_viewport(camera_transform, world_pos.extend(0.))
        .unwrap_or(Vec2::ZERO);

    pos * Vec2::new(1., -1.) + Vec2::Y * window_height
}

pub trait ScreenSize {
    fn screen_size(&self) -> Vec2;
}

impl ScreenSize for Window {
    fn screen_size(&self) -> Vec2 {
        let size = Vec2::new(self.width(), self.height());
        if size.x == 0. || size.y == 0. {
            Vec2::ZERO
        }
        else {
            size
        }
    }
}