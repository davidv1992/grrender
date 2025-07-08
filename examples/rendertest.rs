use std::f64::consts::PI;

use grrender::{
    camera::{BasicCamera, InstantCamera, InstantParallelRayCamera, ParallelRayCamera},
    geometry::{BoundingBox, Coord, FourVector, ManifoldFrame},
    metric::CarthesianMinkowski,
    objects::SphereCollider,
    render::render_scene,
};
use image::Rgb;

fn main() {
    let theta: f64 = 0.25 * PI;
    let beta = 0.999;
    let gamma = 1.0 / (1.0f64 - beta * beta).sqrt();

    let image = render_scene::<CarthesianMinkowski>(
        ParallelRayCamera::new(
            ManifoldFrame {
                root: Coord {
                    components: FourVector([0.0, theta.sin() * 3.0, 0.0, theta.cos() * -3.0]),
                    _metric: std::marker::PhantomData,
                },
                axis: [
                    FourVector([1.0, 0.0, 0.0, 0.0]),
                    FourVector([0.0, theta.cos(), 0.0, theta.sin()]),
                    FourVector([0.0, 0.0, 1.0, 0.0]),
                    FourVector([0.0, -theta.sin(), 0.0, theta.cos()]),
                ],
            },
            400,
            400,
            3.0,
        ),
        vec![(
            Rgb([255, 255, 255]),
            Box::new(SphereCollider {
                center: ManifoldFrame {
                    root: Coord {
                        components: FourVector([-3.0, 0.0, 0.0, 0.0]),
                        _metric: std::marker::PhantomData,
                    },
                    axis: [
                        FourVector([gamma, -beta * gamma, 0.0, 0.0]),
                        FourVector([-beta * gamma, gamma, 0.0, 0.0]),
                        FourVector([0.0, 0.0, 1.0, 0.0]),
                        FourVector([0.0, 0.0, 0.0, 1.0]),
                    ],
                },
                radius: 1.0,
                time_thickness: 10.0,
            }),
        )],
        BoundingBox {
            bbox: [[-7.0, 1.0], [-4.0, 4.0], [-4.0, 4.0], [-4.0, 4.0]],
            _metric: std::marker::PhantomData,
        },
        0.01,
    );
    image.save("render.png").unwrap();
}
