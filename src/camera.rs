use crate::{
    geometry::{Coord, FourVector, ManifoldFrame, ManifoldVector, SpatialVec},
    metric::{CarthesianMinkowski, Metric},
};

pub trait Camera<T: Metric + ?Sized> {
    fn screen_size(&self) -> (usize, usize);
    /// Which direction the ray should go. [0,0,1] corresponds to the "main" camera direction
    fn ray(&self, pixel: (usize, usize)) -> ManifoldVector<T>;
}

pub struct BasicCamera<T: Metric + ?Sized> {
    frame: ManifoldFrame<T>,
    pix_width: usize,
    pix_height: usize,
    plane_width: f64,
    plane_height: f64,
}

impl<T: Metric + ?Sized> BasicCamera<T> {
    pub fn new(frame: ManifoldFrame<T>, width: usize, height: usize, scale: f64) -> Self {
        BasicCamera {
            frame,
            pix_width: width - 1,
            pix_height: height - 1,
            plane_width: scale,
            plane_height: scale * (height as f64) / (width as f64),
        }
    }
}

impl<T: Metric + ?Sized> Camera<T> for BasicCamera<T> {
    fn screen_size(&self) -> (usize, usize) {
        (self.pix_width + 1, self.pix_height + 1)
    }

    fn ray(&self, pixel: (usize, usize)) -> ManifoldVector<T> {
        let x: f64 = (pixel.0 as f64) / (self.pix_width as f64) - 0.5;
        let y: f64 = (pixel.1 as f64) / (self.pix_height as f64) - 0.5;

        let dir = SpatialVec([x * self.plane_width, y * self.plane_height, 1.0]);
        T::from_local(self.frame, CarthesianMinkowski::lightray(dir))
    }
}

pub struct InstantCamera<T: Metric + ?Sized> {
    frame: ManifoldFrame<T>,
    pix_width: usize,
    pix_height: usize,
    plane_width: f64,
    plane_height: f64,
}

impl<T: Metric + ?Sized> InstantCamera<T> {
    pub fn new(frame: ManifoldFrame<T>, width: usize, height: usize, scale: f64) -> Self {
        InstantCamera {
            frame,
            pix_width: width - 1,
            pix_height: height - 1,
            plane_width: scale,
            plane_height: scale * (height as f64) / (width as f64),
        }
    }
}

impl<T: Metric + ?Sized> Camera<T> for InstantCamera<T> {
    fn screen_size(&self) -> (usize, usize) {
        (self.pix_width + 1, self.pix_height + 1)
    }

    fn ray(&self, pixel: (usize, usize)) -> ManifoldVector<T> {
        let x: f64 = (pixel.0 as f64) / (self.pix_width as f64) - 0.5;
        let y: f64 = (pixel.1 as f64) / (self.pix_height as f64) - 0.5;

        let dir = SpatialVec([x * self.plane_width, y * self.plane_height, 1.0]);
        T::from_local(self.frame, CarthesianMinkowski::instantray(dir))
    }
}

pub struct ParallelRayCamera<T: Metric + ?Sized> {
    frame: ManifoldFrame<T>,
    pix_width: usize,
    pix_height: usize,
    plane_width: f64,
    plane_height: f64,
}

impl<T: Metric + ?Sized> ParallelRayCamera<T> {
    pub fn new(frame: ManifoldFrame<T>, width: usize, height: usize, scale: f64) -> Self {
        ParallelRayCamera {
            frame,
            pix_width: width - 1,
            pix_height: height - 1,
            plane_width: scale,
            plane_height: scale * (height as f64) / (width as f64),
        }
    }
}

impl<T: Metric + ?Sized> Camera<T> for ParallelRayCamera<T> {
    fn screen_size(&self) -> (usize, usize) {
        (self.pix_width + 1, self.pix_height + 1)
    }

    fn ray(&self, pixel: (usize, usize)) -> ManifoldVector<T> {
        let x: f64 = (pixel.0 as f64) / (self.pix_width as f64) - 0.5;
        let y: f64 = (pixel.1 as f64) / (self.pix_height as f64) - 0.5;

        T::from_local(
            self.frame,
            ManifoldVector {
                root: Coord {
                    components: FourVector([0.0, x * self.plane_width, y * self.plane_height, 0.0]),
                    _metric: std::marker::PhantomData,
                },
                components: FourVector([-1.0, 0.0, 0.0, 1.0]),
            },
        )
    }
}

pub struct InstantParallelRayCamera<T: Metric + ?Sized> {
    frame: ManifoldFrame<T>,
    pix_width: usize,
    pix_height: usize,
    plane_width: f64,
    plane_height: f64,
}

impl<T: Metric + ?Sized> InstantParallelRayCamera<T> {
    pub fn new(frame: ManifoldFrame<T>, width: usize, height: usize, scale: f64) -> Self {
        InstantParallelRayCamera {
            frame,
            pix_width: width - 1,
            pix_height: height - 1,
            plane_width: scale,
            plane_height: scale * (height as f64) / (width as f64),
        }
    }
}

impl<T: Metric + ?Sized> Camera<T> for InstantParallelRayCamera<T> {
    fn screen_size(&self) -> (usize, usize) {
        (self.pix_width + 1, self.pix_height + 1)
    }

    fn ray(&self, pixel: (usize, usize)) -> ManifoldVector<T> {
        let x: f64 = (pixel.0 as f64) / (self.pix_width as f64) - 0.5;
        let y: f64 = (pixel.1 as f64) / (self.pix_height as f64) - 0.5;

        T::from_local(
            self.frame,
            ManifoldVector {
                root: Coord {
                    components: FourVector([0.0, x * self.plane_width, y * self.plane_height, 0.0]),
                    _metric: std::marker::PhantomData,
                },
                components: FourVector([0.0, 0.0, 0.0, 1.0]),
            },
        )
    }
}
