use super::Constraint;

/// A Eucledian ball
pub struct Ball2<'a> {
    center: Option<&'a [f64]>,
    radius: f64,
}

impl<'a> Ball2<'a> {
    /// Construct a new Eucledian ball with given center and radius
    pub fn new(center: Option<&'a [f64]>, radius: f64) -> Ball2<'a> {
        assert!(radius > 0.0);

        Ball2 { center, radius }
    }
}

impl<'a> Constraint for Ball2<'a> {
    fn project(&self, x: &mut [f64]) {
        if let Some(center) = &self.center {
            let mut norm_difference = 0.0;
            x.iter().zip(center.iter()).for_each(|(a, b)| {
                let diff_ = *a - *b;
                norm_difference += diff_ * diff_
            });

            norm_difference = norm_difference.sqrt();

            if norm_difference > self.radius {
                x.iter_mut().zip(center.iter()).for_each(|(x, c)| {
                    *x = *c + (*x - *c) / norm_difference;
                });
            }
        } else {
            let norm_x = crate::matrix_operations::norm2(x);
            if norm_x > self.radius {
                let norm_over_radius = norm_x / self.radius;
                x.iter_mut().for_each(|x_| *x_ /= norm_over_radius);
            }
        }
    }
}
