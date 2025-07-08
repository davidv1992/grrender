use image::{Rgb, RgbImage};

use crate::{camera::Camera, geometry::BoundingBox, metric::Metric, objects::RayIntersector};

pub fn render_scene<T: Metric + ?Sized>(
    camera: impl Camera<T>,
    objects: Vec<(Rgb<u8>, Box<dyn RayIntersector<T>>)>,
    bounds: BoundingBox<T>,
    step: f64,
) -> RgbImage {
    let (width, height) = camera.screen_size();
    RgbImage::from_fn(width as _, height as _, |x, y| {
        let mut lightray = camera.ray((x as _, y as _));
        while bounds.contains(lightray.root) {
            lightray = T::step_geodesic(lightray, step);

            for (color, object) in &objects {
                if object.intersects(lightray, step) {
                    return *color;
                }
            }
        }
        Rgb([0, 0, 0])
    })
}
