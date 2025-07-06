use std::{array, iter::Sum, marker::PhantomData, ops::{Add, AddAssign, Mul, Sub, SubAssign}};

use crate::metric::Metric;

/// Basic 4-vector with math operations
#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub struct FourVector(pub [f64; 4]);

impl Mul<FourVector> for f64 {
    type Output = FourVector;

    fn mul(self, rhs: FourVector) -> Self::Output {
        FourVector(array::from_fn(|i| rhs.0[i] * self))
    }
}

impl Add<FourVector> for FourVector {
    type Output = FourVector;

    fn add(self, rhs: FourVector) -> Self::Output {
        FourVector(array::from_fn(|i| self.0[i] + rhs.0[i]))
    }
}

impl AddAssign<FourVector> for FourVector {
    fn add_assign(&mut self, rhs: FourVector) {
        *self = *self + rhs;
    }
}

impl Sub<FourVector> for FourVector {
    type Output = FourVector;

    fn sub(self, rhs: FourVector) -> Self::Output {
        FourVector(array::from_fn(|i| self.0[i] - rhs.0[i]))
    }
}

impl SubAssign<FourVector> for FourVector {
    fn sub_assign(&mut self, rhs: FourVector) {
        *self = *self - rhs;
    }
}

impl Sum<FourVector> for FourVector {
    fn sum<I: Iterator<Item = FourVector>>(iter: I) -> Self {
        let mut acc = FourVector::default();
        for v in iter {
            acc += v;
        }
        acc
    }
}

/// Coordinate on a manifold
#[derive(PartialEq, Debug)]
pub struct Coord<T: Metric + ?Sized> {
    pub components: FourVector,
    pub _metric: PhantomData<T>,
}

impl<T: Metric + ?Sized> Clone for Coord<T> {
    fn clone(&self) -> Self {
        Self {
            components: self.components.clone(),
            _metric: PhantomData,
        }
    }
}

impl<T: Metric + ?Sized> Copy for Coord<T> {}

/// Vector in the tangent space of a manifold
#[derive(PartialEq, Debug)]
pub struct ManifoldVector<T: Metric + ?Sized> {
    pub root: Coord<T>,
    pub components: FourVector,
}

impl<T: Metric + ?Sized> Clone for ManifoldVector<T> {
    fn clone(&self) -> Self {
        Self {
            root: self.root,
            components: self.components,
        }
    }
}

impl<T: Metric + ?Sized> Copy for ManifoldVector<T> {}

#[derive(PartialEq, Debug)]
pub struct ManifoldFrame<T: Metric + ?Sized> {
    pub root: Coord<T>,
    pub axis: [FourVector; 4],
}

impl<T: Metric + ?Sized> ManifoldFrame<T> {
    pub fn normal(self) -> bool {
        const EPS: f64 = 1e-10;
        if (T::norm(ManifoldVector { root: self.root, components: self.axis[0]}) + 1.0).abs() > EPS {
            return false;
        }
        for i in 1..4 {
            if (T::norm(ManifoldVector { root: self.root, components: self.axis[i] }) - 1.0).abs() > EPS {
                return false;
            }
        }
        for i in 0..4 {
            for j in i+1..4 {
                if T::inner(self.root, self.axis[i], self.axis[j]).abs() > EPS {
                    return false;
                }
            }
        }
        true
    }

    pub fn normalize(self) -> Self {
        let t = (1.0 / T::norm(ManifoldVector { root: self.root, components: self.axis[0] }).abs().sqrt()) * self.axis[0];
        let x = self.axis[1] + T::inner(self.root, t, self.axis[1]) * t;
        let x = (1.0 / T::norm(ManifoldVector { root: self.root, components: x}).abs().sqrt()) * x;
        let y = self.axis[2] + T::inner(self.root, t, self.axis[2]) * t - T::inner(self.root, x, self.axis[2]) * x;
        let y = (1.0 / T::norm(ManifoldVector { root: self.root, components: y }).abs().sqrt()) * y;
        let z = self.axis[3] + T::inner(self.root, t, self.axis[3]) * t - T::inner(self.root, x, self.axis[3]) * x - T::inner(self.root, y, self.axis[3]) * y;
        let z = (1.0 / T::norm(ManifoldVector { root: self.root, components: z }).abs().sqrt()) * z;
        ManifoldFrame { root: self.root, axis: [t, x, y, z] }
    }
}

impl<T: Metric + ?Sized> Clone for ManifoldFrame<T> {
    fn clone(&self) -> Self {
        Self {
            root: self.root,
            axis: self.axis,
        }
    }
}

impl<T: Metric + ?Sized> Copy for ManifoldFrame<T> {}

/// Vector describing relative distances in flat 3-dimensional euclidean space
#[derive(PartialEq, Debug)]
pub struct SpatialVec(pub [f64; 3]);

#[derive(Debug, PartialEq)]
pub struct BoundingBox<T: Metric + ?Sized> {
    pub bbox: [[f64; 2]; 4],
    pub _metric: PhantomData<T>,
}

impl<T: Metric + ?Sized> Clone for BoundingBox<T> {
    fn clone(&self) -> Self {
        Self {
            bbox: self.bbox.clone(),
            _metric: PhantomData,
        }
    }
}

impl<T: Metric + ?Sized> Copy for BoundingBox<T> {}

impl<T: Metric + ?Sized> BoundingBox<T> {
    pub fn contains(self, point: Coord<T>) -> bool {
        (0..4).into_iter().all(|i| self.bbox[i][0] <= point.components.0[i] && self.bbox[i][1] >= point.components.0[i])
    }
}