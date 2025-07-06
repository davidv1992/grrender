use grrender::{
    camera::BasicCamera,
    geometry::{BoundingBox, Coord, FourVector, ManifoldFrame},
    metric::CarthesianMinkowski,
    objects::SphereCollider,
    render::render_scene,
};
use image::Rgb;

fn main() {
    let beta = 0.9999;
    let gamma = 1.0 / (1.0f64 - beta * beta).sqrt();

    let image = render_scene::<CarthesianMinkowski>(
        BasicCamera::new(400, 400, 1.0),
        ManifoldFrame {
            root: Coord {
                components: FourVector([0.0; 4]),
                _metric: std::marker::PhantomData,
            },
            axis: [
                FourVector([1.0, 0.0, 0.0, 0.0]),
                FourVector([0.0, 1.0, 0.0, 0.0]),
                FourVector([0.0, 0.0, 1.0, 0.0]),
                FourVector([0.0, 0.0, 0.0, 1.0]),
            ],
        },
        vec![(
            Rgb([255, 255, 255]),
            Box::new(SphereCollider {
                center: ManifoldFrame {
                    root: Coord {
                        components: FourVector([-3.0, 0.0, 0.0, 3.0]),
                        _metric: std::marker::PhantomData,
                    },
                    axis: [
                        FourVector([gamma, -beta * gamma, 0.0, 0.0]),
                        FourVector([- beta * gamma, gamma, 0.0, 0.0]),
                        FourVector([0.0, 0.0, 1.0, 0.0]),
                        FourVector([0.0, 0.0, 0.0, 1.0]),
                    ],
                },
                radius: 1.0,
                time_thickness: 10.0,
            }),
        )],
        BoundingBox {
            bbox: [[-7.0, 1.0], [-3.0, 3.0], [-3.0, 3.0], [-1.0, 5.0]],
            _metric: std::marker::PhantomData,
        },
        0.01,
    );
    image.save("render.png").unwrap();
}
