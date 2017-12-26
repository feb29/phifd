mod double_exp;

pub use self::double_exp::DoubleExp;

pub fn double_exp(a: f64, b: f64) -> DoubleExp {
    DoubleExp::new(a, b)
}

pub trait Smooth {
    fn apply(&mut self, v: f64) -> f64;
}

impl<F> Smooth for F
where
    F: FnMut(f64) -> f64,
{
    fn apply(&mut self, v: f64) -> f64 {
        (self)(v)
    }
}
