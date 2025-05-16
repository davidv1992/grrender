use std::marker::PhantomData;

use crate::metric::Metric;

/// Coordinate on a manifold
#[derive(PartialEq)]
pub struct Coord<T: Metric + ?Sized> {
    pub components: [f64; 4],
    pub _metric: PhantomData<T>,
}

impl<T: Metric> Clone for Coord<T> {
    fn clone(&self) -> Self {
        Self { components: self.components.clone(), _metric: self._metric.clone() }
    }
}

impl<T: Metric> Copy for Coord<T> {}

/// Vector in the tangent space of a manifold
#[derive(PartialEq)]
pub struct ManifoldVector<T: Metric + ?Sized> {
    pub root: Coord<T>,
    pub components: [f64; 4],
}

impl<T: Metric> Clone for ManifoldVector<T> {
    fn clone(&self) -> Self {
        Self { root: self.root.clone(), components: self.components.clone() }
    }
}

impl<T: Metric> Copy for ManifoldVector<T> {}

/// Vector describing relative distances in flat 3-dimensional euclidean space
pub struct SpatialVec(pub [f64; 3]);
