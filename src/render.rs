use image::{Rgb, RgbImage};

use crate::{camera::Camera, geometry::{BoundingBox, ManifoldFrame}, metric::{CarthesianMinkowski, Metric}, objects::RayIntersector};

pub fn render_scene<T: Metric + ?Sized>(camera: impl Camera, camera_pos: ManifoldFrame<T>, objects: Vec<(Rgb<u8>, Box<dyn RayIntersector<T>>)>, bounds: BoundingBox<T>, step: f64) -> RgbImage {
    let (width, height) = camera.screen_size();
    RgbImage::from_fn(width as _, height as _, |x, y| {
        let dir = camera.relative_dir((x as _, y as _));
        let mut lightray = T::from_local(camera_pos, CarthesianMinkowski::lightray(dir));
        while bounds.contains(lightray.root) {
            lightray = T::step_geodesic(lightray, step);

            for (color, object) in &objects {
                if object.intersects(lightray, step) {
                    return *color;
                }
            }
        }
        Rgb([0,0,0])
    })
}