/// `Holt's Linear Method` (DoubleExponentialSmoothing),
///  good for non-seasonal data with a trend.
///
///    Level:  L[i]   = aX[i] + (1−a)(L[i-1] + T[i-1])
///    Trend:  T[i]   = b(L[i] − L[i−1]) + (1−b)T[i−1]
///    Fitted: F[i+1] = L[i] + T[i] (a fitted value, or one-step-ahead forecast, at time)
pub struct DoubleExp {
    inner: Inner,
}

fn compute(a: f64, b: f64, level: f64, trend: f64, value: f64) -> (f64, f64) {
    let new_level = a * value + (1.0 - a) * (level + trend);
    let new_trend = b * (new_level - level) + (1.0 - b) * trend;
    (new_level, new_trend)
}

impl DoubleExp {
    /// a and b are factor for smoothing observed signals.
    pub fn new(a: f64, b: f64) -> Self {
        let inner = Inner::Init0 { a, b };
        Self { inner }
    }

    pub fn level(&self) -> Option<f64> {
        self.inner.level()
    }
    pub fn trend(&self) -> Option<f64> {
        self.inner.trend()
    }
}

impl super::Smooth for DoubleExp {
    fn apply(&mut self, v: f64) -> f64 {
        self.inner.update(v)
    }
}

enum Inner {
    Init0 {
        a: f64,
        b: f64,
    },
    Init1 {
        a: f64,
        b: f64,
        i0: f64,
    },
    Yield {
        a: f64,
        b: f64,
        level: f64,
        trend: f64,
    },
}

impl Inner {
    // fn init0(&self) -> Self {
    //     match *self {
    //         Inner::Init0 { a, b } => Inner::Init0 { a, b },
    //         Inner::Init1 { a, b, .. } => Inner::Init0 { a, b },
    //         Inner::Yield { a, b, .. } => Inner::Init0 { a, b },
    //     }
    // }

    fn update(&mut self, value: f64) -> f64 {
        match *self {
            Inner::Init0 { a, b } => {
                *self = Inner::Init1 { a, b, i0: value };
                value
            }
            Inner::Init1 { a, b, i0 } => {
                let (level, trend) = compute(a, b, i0, value - i0, value);
                *self = Inner::Yield { a, b, level, trend };
                level + trend
            }
            Inner::Yield {
                a,
                b,
                ref mut level,
                ref mut trend,
            } => {
                let (lv, tr) = compute(a, b, *level, *trend, value);
                *level = lv;
                *trend = tr;
                lv + tr
            }
        }
    }

    fn level(&self) -> Option<f64> {
        match *self {
            Inner::Init0 { .. } | Inner::Init1 { .. } => None,
            Inner::Yield { level, .. } => Some(level),
        }
    }

    fn trend(&self) -> Option<f64> {
        match *self {
            Inner::Init0 { .. } | Inner::Init1 { .. } => None,
            Inner::Yield { trend, .. } => Some(trend),
        }
    }

    // fn last(&self) -> Option<f64> {
    //     match *self {
    //         Inner::Init0 { .. } | Inner::Init1 { .. } => None,
    //         Inner::Yield { level, trend, .. } => Some(level + trend),
    //     }
    // }
}
