use crate::{
    geometry::{bounded_volume_hierarchy::BvhNode, vector_3d::Vector3D},
    util::{camera::Camera, point::Point3D},
};

use super::world;

pub struct Scene {
    pub camera: Camera,
    pub objects: BvhNode,
}

impl Scene {
    pub fn two_balls(aspect_ratio: f64) -> Scene {
        todo!()
    }

    pub fn random_scene(aspect_ratio: f64) -> Scene {
        let objects = BvhNode::new(world::random_scene(), 0.0, 1.0);

        // Camera
        let look_from = Point3D::new(13.0, 2.0, 3.0);
        let look_at = Point3D::new(0.0, 0.0, 0.0);
        let v_up = Vector3D::new(0.0, 1.0, 0.0);
        let dist_to_focus = 10.0;
        let aperature = 0.1;
        let camera = Camera::new(
            look_from,
            look_at,
            v_up,
            20.0,
            aspect_ratio,
            aperature,
            dist_to_focus,
            0.0,
            1.0,
        );

        Scene { camera, objects }
    }
}
