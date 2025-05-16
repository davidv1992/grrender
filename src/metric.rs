use std::{array, marker::PhantomData};

use crate::{
    geometry::{Coord, ManifoldVector},
    util::sqr,
};

pub trait Metric {
    fn step_geodesic(start: ManifoldVector<Self>, step: f64) -> ManifoldVector<Self>;

    fn into_local(
        root: ManifoldVector<Self>,
        secondary: ManifoldVector<Self>,
    ) -> ManifoldVector<CarthesianMinkowski>;
}

pub struct CarthesianMinkowski;

impl Metric for CarthesianMinkowski {
    fn step_geodesic(start: ManifoldVector<Self>, step: f64) -> ManifoldVector<Self> {
        ManifoldVector {
            root: Coord {
                components: array::from_fn(|i| {
                    start.root.components[i] + start.components[i] * step
                }),
                _metric: PhantomData,
            },
            components: start.components,
        }
    }

    fn into_local(
        root: ManifoldVector<Self>,
        secondary: ManifoldVector<Self>,
    ) -> ManifoldVector<CarthesianMinkowski> {
        assert!(
            -sqr(root.components[0])
                + sqr(root.components[1])
                + sqr(root.components[2])
                + sqr(root.components[3])
                < 0.0
        );
        let beta: [_; 3] = array::from_fn(|i| root.components[1 + i] / root.components[0]);
        let gamma = 1. / ((1. - beta.iter().map(|v| sqr(*v)).sum::<f64>()).sqrt());
        let gamma2 = sqr(gamma) / (1. + gamma);

        ManifoldVector {
            root: Coord {
                components: [
                    gamma * secondary.root.components[0]
                        - (0..3)
                            .map(|i| gamma * beta[i] * secondary.root.components[i + 1])
                            .sum::<f64>(),
                    -gamma * beta[0] * secondary.root.components[0]
                        + (0..3)
                            .map(|i| gamma2 * beta[i] * beta[0] * secondary.root.components[i + 1])
                            .sum::<f64>()
                        + secondary.root.components[0],
                    -gamma * beta[1] * secondary.root.components[0]
                        + (0..3)
                            .map(|i| gamma2 * beta[i] * beta[1] * secondary.root.components[i + 1])
                            .sum::<f64>()
                        + secondary.root.components[1],
                    -gamma * beta[2] * secondary.root.components[0]
                        + (0..3)
                            .map(|i| gamma2 * beta[i] * beta[2] * secondary.root.components[i + 1])
                            .sum::<f64>()
                        + secondary.root.components[2],
                ],
                _metric: PhantomData,
            },
            components: [
                gamma * secondary.components[0]
                    - (0..3)
                        .map(|i| gamma * beta[i] * secondary.components[i + 1])
                        .sum::<f64>(),
                -gamma * beta[0] * secondary.components[0]
                    + (0..3)
                        .map(|i| gamma2 * beta[i] * beta[0] * secondary.components[i + 1])
                        .sum::<f64>()
                    + secondary.components[0],
                -gamma * beta[1] * secondary.components[0]
                    + (0..3)
                        .map(|i| gamma2 * beta[i] * beta[1] * secondary.components[i + 1])
                        .sum::<f64>()
                    + secondary.components[1],
                -gamma * beta[2] * secondary.components[0]
                    + (0..3)
                        .map(|i| gamma2 * beta[i] * beta[2] * secondary.components[i + 1])
                        .sum::<f64>()
                    + secondary.components[2],
            ],
        }
    }
}
