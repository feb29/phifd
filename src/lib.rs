//! `φ accrual failure detector`.
//! The value of φ is calculated as follows.
//!
//!    value(T[now]) = −log10(P-later(T[now] − T[last]))
//!                  = −log10(1 - F(T[now] − T[last]))
//!                  = −log10(1 - F(timeSinceLastHeartbeat))
//!
//! F(t) is the cumulative distribution function of a normal distribution
//! with mean and variance estimated from historical heartbeat inter-arrival durations.
//!
//! Refereces:
//!   - [http://www.jaist.ac.jp/~defago/files/pdf/IS_RR_2004_010.pdf]
//!   - [https://dspace.jaist.ac.jp/dspace/bitstream/10119/4799/1/IS-RR-2007-007.pdf]

#![feature(conservative_impl_trait)]

mod smoothing;

use std::collections::VecDeque;

pub use smoothing::{double_exp, DoubleExp, Smooth};

pub struct AccrualFailureDetector<F> {
    smooth: F,
    window: Window,
}

impl<T: Smooth> AccrualFailureDetector<T> {
    pub fn new(cap: usize, smooth: T) -> Self {
        let window = Window::new(cap);
        Self { smooth, window }
    }

    pub fn observe(&mut self, x: f64) {
        let value = self.smooth.apply(x);
        self.window.push(value);
    }

    pub fn failure(&self, x: f64) -> f64 {
        let (mean, stddev) = self.window.mean_stddev();
        value(mean, stddev, x)
    }
}

fn value(mean: f64, stddev: f64, x: f64) -> f64 {
    let y = (x - mean) / stddev;
    let p = (-y * (1.5976 + 0.070566 * y * y)).exp();
    let log10 = if x > mean {
        (p / (1. + p)).log10()
    } else {
        (1. - 1. / (1. + p)).log10()
    };
    -log10
}

struct Window {
    // container that holds observed values
    vec: VecDeque<f64>,
    // max size of this window
    max: usize,
    // sum of values
    sum: f64,
    // sum of square
    sos: f64,
}

impl Window {
    fn new(max: usize) -> Self {
        let vec = VecDeque::with_capacity(max);
        let (sum, sos) = (0.0, 0.0);
        Window { vec, max, sum, sos }
    }

    fn push(&mut self, x: f64) {
        self.vec.push_back(x);
        self.sum += x;
        self.sos += x * x;
        while self.max < self.vec.len() {
            self.pop();
        }
    }

    fn pop(&mut self) {
        if let Some(x) = self.vec.pop_front() {
            self.sum -= x;
            self.sos -= x * x;
        }
    }

    fn mean_stddev(&self) -> (f64, f64) {
        let len = self.vec.len() as f64;
        let mean = self.sum / len;
        let vars = self.sos / len - mean * mean;
        let stddev = vars.sqrt();
        (mean, stddev)
    }
}
