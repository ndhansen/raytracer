use rand::Rng;

use crate::{
    geometry::vector_3d::Vector3D,
    util::{color::Color, point::Point3D},
};

use super::{
    hittable::Hittable,
    materials::{Dielectric, Lambertian, Material, Metal},
    moving_sphere::MovingSphere,
    sphere::Sphere,
};

pub fn random_scene() -> Vec<Box<dyn Hittable>> {
    let mut world: Vec<Box<dyn Hittable>> = vec![];

    let ground_material = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(
        Point3D::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let mut generator = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choosen_material = generator.gen_range(0.0..0.5);
            let center = Point3D::new(
                (a as f64) + 0.9 * generator.gen_range(0.0..1.0),
                0.2,
                (b as f64) + 0.9 * generator.gen_range(0.0..1.0),
            );

            if (center - Point3D::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Box<dyn Material> = if choosen_material < 0.8 {
                    // diffuse
                    let albedo = Color::all_random() * Color::all_random();
                    Box::new(Lambertian::new(albedo))
                } else if choosen_material < 0.95 {
                    let albedo = Color::random(0.5, 1.0);
                    let fuzz = generator.gen_range(0.0..0.5);
                    Box::new(Metal::new(albedo, fuzz))
                } else {
                    Box::new(Dielectric::new(1.5))
                };

                let sphere: Box<dyn Hittable> = if generator.gen_range(0.0..1.0) > 0.5 {
                    let target_center =
                        center + Vector3D::new(0.0, generator.gen_range(0.0..0.5), 0.0);
                    Box::new(MovingSphere::new(
                        center,
                        target_center,
                        0.2,
                        sphere_material,
                        0.0,
                        1.0,
                    ))
                } else {
                    Box::new(Sphere::new(center, 0.2, sphere_material))
                };
                world.push(sphere);
            }
        }
    }

    let dielectric = Box::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(
        Point3D::new(0.0, 1.0, 0.0),
        1.0,
        dielectric,
    )));

    let lambertian = Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(
        Point3D::new(-4.0, 1.0, 0.0),
        1.0,
        lambertian,
    )));

    let metal = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(
        Point3D::new(4.0, 1.0, 0.0),
        1.0,
        metal,
    )));

    world
}
