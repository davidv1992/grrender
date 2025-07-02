use std::{array, marker::PhantomData};

use crate::{
    geometry::{Coord, FourVector, ManifoldFrame, ManifoldVector},
};

pub trait Metric {
    fn step_geodesic(start: ManifoldVector<Self>, step: f64) -> ManifoldVector<Self>;

    fn norm(vector: ManifoldVector<Self>) -> f64;

    fn inner(root: Coord<Self>, a: FourVector, b: FourVector) -> f64;

    fn into_local(
        root: ManifoldFrame<Self>,
        secondary: ManifoldVector<Self>,
    ) -> ManifoldVector<CarthesianMinkowski> {
        let posdelta = secondary.root.components - root.root.components;

        ManifoldVector {
            root: Coord {
                components: FourVector(array::from_fn(|i| {
                    Self::inner(root.root, root.axis[i], posdelta)
                        / Self::norm(ManifoldVector {
                            root: root.root,
                            components: root.axis[i],
                        })
                })),
                _metric: PhantomData,
            },
            components: FourVector(array::from_fn(|i| {
                Self::inner(root.root, root.axis[i], secondary.components)
                    / Self::norm(ManifoldVector {
                        root: root.root,
                        components: root.axis[i],
                    })
            })),
        }
    }

    fn from_local(
        frame: ManifoldFrame<Self>,
        vector: ManifoldVector<CarthesianMinkowski>,
    ) -> ManifoldVector<Self>;
}

pub struct CarthesianMinkowski;

impl CarthesianMinkowski {
    const METRIC: [f64; 4] = [-1.0, 1.0, 1.0, 1.0];
}

impl Metric for CarthesianMinkowski {
    fn step_geodesic(start: ManifoldVector<Self>, step: f64) -> ManifoldVector<Self> {
        ManifoldVector {
            root: Coord {
                components: start.root.components + step * start.components,
                _metric: PhantomData,
            },
            components: start.components,
        }
    }

    fn norm(vector: ManifoldVector<Self>) -> f64 {
        (0..4)
            .into_iter()
            .map(|i| vector.components.0[i].powi(2) * Self::METRIC[i])
            .sum()
    }

    fn inner(_root: Coord<Self>, a: FourVector, b: FourVector) -> f64 {
        (0..4)
            .into_iter()
            .map(|i| a.0[i] * b.0[i] * Self::METRIC[i])
            .sum()
    }

    fn from_local(
        frame: ManifoldFrame<Self>,
        vector: ManifoldVector<CarthesianMinkowski>,
    ) -> ManifoldVector<Self> {
        ManifoldVector {
            root: Coord {
                components: frame.root.components
                    + (0..4)
                        .into_iter()
                        .map(|i| vector.root.components.0[i] * frame.axis[i])
                        .sum(),
                _metric: PhantomData,
            },
            components: (0..4)
                .into_iter()
                .map(|i| vector.components.0[i] * frame.axis[i])
                .sum(),
        }
    }
}
