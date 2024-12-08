use super::ODEState;
use crate::Vector;

impl<const M: usize> ODEState for Vector<M> {
    #[inline(always)]
    fn ode_elem_div(&self, other: &Self) -> Self {
        self.iter()
            .zip(other.iter())
            .map(|(x, y)| *x / *y)
            .collect::<Vector<M>>()
    }

    #[inline(always)]
    fn ode_elem_max(&self, other: &Self) -> Self {
        self.iter()
            .zip(other.iter())
            .map(|(x, y)| x.max(*y))
            .collect::<Vector<M>>()
    }

    #[inline(always)]
    fn ode_norm(&self) -> f64 {
        self.norm() / (self.ode_nelem() as f64).sqrt()
    }

    #[inline(always)]
    fn ode_abs(&self) -> Self {
        self.iter().map(|x| x.abs()).collect::<Vector<M>>()
    }

    #[inline(always)]
    fn ode_scalar_add(&self, s: f64) -> Self {
        self.iter().map(|x| *x + s).collect::<Vector<M>>()
    }

    #[inline(always)]
    fn ode_nelem(&self) -> usize {
        self.cols() * self.rows()
    }

    fn ode_zero() -> Self {
        Self::zeros()
    }
}
