use std::marker::PhantomData;

use crate::{
    geometry::{BoundingBox, Coord, FourVector, ManifoldFrame, ManifoldVector},
    metric::Metric,
    util::sqr,
};

pub trait RayIntersector<T: Metric> {
    fn intersects(&self, ray: ManifoldVector<T>) -> bool;

    fn in_bounding_box(&self, bbox: BoundingBox<T>) -> bool;
}

pub struct SphereCollider<T: Metric> {
    pub center: ManifoldFrame<T>,
    pub radius: f64,
    pub time_thickness: f64,
}

impl<T: Metric> RayIntersector<T> for SphereCollider<T> {
    fn intersects(&self, ray: ManifoldVector<T>) -> bool {
        let local_ray = T::into_local(self.center, ray);

        let ab = (0..3)
            .map(|i| local_ray.root.components.0[i + 1] * local_ray.components.0[i + 1])
            .sum::<f64>();
        let na = (0..3)
            .map(|i| sqr(local_ray.root.components.0[i + 1]))
            .sum::<f64>();
        let nb = (0..3).map(|i| sqr(local_ray.components.0[i])).sum::<f64>();
        let r2 = sqr(self.radius);

        let d = sqr(ab) - na * nb + nb * r2;

        if d < 0.0 {
            return false;
        }

        let rootd = d.sqrt();

        let t1 = (-ab + rootd) / sqr(nb);
        let t2 = (-ab - rootd) / sqr(nb);

        let tau1 = local_ray.root.components.0[0] + t1 * local_ray.components.0[0];
        let tau2 = local_ray.root.components.0[0] + t2 * local_ray.components.0[0];

        tau1.abs() <= self.time_thickness || tau2.abs() <= self.time_thickness
    }

    fn in_bounding_box(&self, bbox: BoundingBox<T>) -> bool {
        let mut upper = [f64::MIN; 4];
        let mut lower = [f64::MAX; 4];
        for i0 in 0..1 {
            for i1 in 0..1 {
                for i2 in 0..1 {
                    for i3 in 0..1 {
                        let local = T::into_local(
                            self.center,
                            ManifoldVector {
                                root: Coord {
                                    components: FourVector([
                                        bbox.bbox[0][i0],
                                        bbox.bbox[1][i1],
                                        bbox.bbox[2][i2],
                                        bbox.bbox[3][i3],
                                    ]),
                                    _metric: PhantomData,
                                },
                                components: FourVector::default(),
                            },
                        );
                        for i in 0..4 {
                            upper[i] = upper[i].max(local.root.components.0[i]);
                            lower[i] = lower[i].min(local.root.components.0[i]);
                        }
                    }
                }
            }
        }

        lower[0] < 2.0 * self.time_thickness
            && upper[0] > -2.0 * self.time_thickness
            && (lower[1].abs().min(upper[1].abs()) < 2.0 * self.radius
                || lower[1].signum() != upper[1].signum())
            && (lower[1].abs().min(upper[2].abs()) < 2.0 * self.radius
                || lower[2].signum() != upper[2].signum())
            && (lower[1].abs().min(upper[3].abs()) < 2.0 * self.radius
                || lower[3].signum() != upper[3].signum())
    }
}
