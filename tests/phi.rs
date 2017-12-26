extern crate phifd as phi;
extern crate rand;

use std::time;
use rand::Rng;

#[test]
fn test_double_exp() {
    use phi::Smooth;
    let mut rng = rand::thread_rng();
    let mut exp = phi::double_exp(0.4, 1.0);
    for _ in 0..100 {
        let millis = {
            let duration = time::Duration::from_millis(rng.gen_range(100, 500));
            let secs = duration.as_secs() as f64 * 1000.0;
            let nanos = duration.subsec_nanos() as f64 / 1_000_000.0;
            secs + nanos
        };
        let apply = exp.apply(millis);
        let level = exp.level().unwrap_or(0.0);
        let trend = exp.trend().unwrap_or(0.0);
        eprintln!("{:>10.2} {:>10.2} {:>10.2} {:>10.2}", millis, level, trend, apply);
    }
}

#[test]
fn test_heartbeat() {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(3000);
    for _ in 0..3000 {
        let millis = {
            let duration = time::Duration::from_millis(rng.gen_range(100, 500));
            let secs = duration.as_secs() as f64 * 1000.0;
            let nanos = duration.subsec_nanos() as f64 / 1_000_000.0;
            secs + nanos
        };
        vec.push(millis);
    }

    let phi = phi::AccrualFailureDetector::new(1000, phi::double_exp(0.525, 0.525));
    heartbeat("(0.525, 0.525)", &vec, phi);

    let phi = phi::AccrualFailureDetector::new(1000, phi::double_exp(0.625, 0.625));
    heartbeat("(0.625, 0.625)", &vec, phi);

    let phi = phi::AccrualFailureDetector::new(1000, phi::double_exp(0.725, 0.725));
    heartbeat("(0.725, 0.725)", &vec, phi);

    let phi = phi::AccrualFailureDetector::new(1000, phi::double_exp(0.625, 1.000));
    heartbeat("(0.625, 1.000)", &vec, phi);

    let phi = phi::AccrualFailureDetector::new(1000, phi::double_exp(1.000, 1.000));
    heartbeat("(1.000, 1.000)", &vec, phi);

    let phi = phi::AccrualFailureDetector::new(1000, |x| x);
    heartbeat("no-smooth", &vec, phi);
}

fn heartbeat<T>(name: &'static str, series: &[f64], mut phi: phi::AccrualFailureDetector<T>)
where
    T: phi::Smooth,
{
    for &v in series {
        phi.observe(v);
    }
    for i in 0..25 {
        let v = i as f64 * 100.0;
        eprintln!("{:>16} {:>5} {:>8.4}", name, v, phi.failure(v));
    }
}
