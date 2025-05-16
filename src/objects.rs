use crate::{geometry::ManifoldVector, metric::Metric, util::sqr};

pub trait RayIntersector<T: Metric> {
    fn intersects(&self, ray: ManifoldVector<T>) -> bool;
}

pub struct SphereCollider<T: Metric> {
    pub center: ManifoldVector<T>,
    pub radius: f64,
    pub time_thickness: f64,
}

impl<T: Metric> RayIntersector<T> for SphereCollider<T> {
    fn intersects(&self, ray: ManifoldVector<T>) -> bool {
        let local_ray = T::into_local(self.center, ray);

        let ab = (0..3).map(|i| local_ray.root.components[i+1] * local_ray.components[i+1]).sum::<f64>();
        let na = (0..3).map(|i| sqr(local_ray.root.components[i+1])).sum::<f64>();
        let nb = (0..3).map(|i| sqr(local_ray.components[i])).sum::<f64>();
        let r2 = sqr(self.radius);

        let d = sqr(ab) - na * nb + nb * r2;

        if d < 0.0 {
            return false;
        }

        let rootd = d.sqrt();
        
        let t1 = (-ab + rootd)/sqr(nb);
        let t2 = (-ab - rootd)/sqr(nb);

        let tau1 = local_ray.root.components[0] + t1 * local_ray.components[0];
        let tau2 = local_ray.root.components[0] + t2 * local_ray.components[0];

        tau1.abs() <= self.time_thickness || tau2.abs() <= self.time_thickness
    }
}