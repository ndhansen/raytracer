use std::rc::Rc;

use crate::geometry::ray::Ray;

use super::hittable::{HitRecord, Hittable};

// pub struct HittableList<'list> {
//     objects: Vec<&'list dyn Hittable>,
// }

// impl<'list> HittableList<'list> {
//     fn empty() -> HittableList<'list> {
//         HittableList {
//             objects: Vec::new()
//         }
//     }

//     fn add(&mut self, object: &'list impl Hittable) {
//         self.objects.push(object.clone())
//     }
// }

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn empty() -> HittableList {
        HittableList {
            objects: Vec::new()
        }
    }

    pub fn new(object: &Rc<dyn Hittable>) -> HittableList {
        HittableList {
            objects: vec![object.clone()],
        }
    }

    pub fn add(&mut self, object: &Rc<dyn Hittable>) {
        self.objects.push(object.clone())
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        let mut temp_record = HitRecord::empty();
        for object in &self.objects {
            if object.hit(ray, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *record = temp_record.clone();
            }
        }

        return hit_anything;
    }
}
