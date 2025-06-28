use crate::geometry::SpatialVec;

pub trait Camera {
    fn screen_size(&self) -> (usize, usize);
    /// Which direction the ray should go. [0,0,1] corresponds to the "main" camera direction
    fn relative_dir(&self, pixel: (usize, usize)) -> SpatialVec;
}

pub struct BasicCamera {
    pix_width: usize,
    pix_height: usize,
    plane_width: f64,
    plane_height: f64,
}

impl BasicCamera {
    pub fn new(width: usize, height: usize, scale: f64) -> Self {
        BasicCamera {
            pix_width: width,
            pix_height: height,
            plane_width: scale,
            plane_height: scale * (height as f64) / (width as f64),
        }
    }
}

impl Camera for BasicCamera {
    fn screen_size(&self) -> (usize, usize) {
        (self.pix_width, self.pix_height)
    }

    fn relative_dir(&self, pixel: (usize, usize)) -> SpatialVec {
        let x: f64 = (pixel.0 as f64) / (self.pix_width as f64) - 0.5;
        let y: f64 = (pixel.1 as f64) / (self.pix_height as f64) - 0.5;

        SpatialVec([x * self.plane_width, y * self.plane_height, 1.0])
    }
}
