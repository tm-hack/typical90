use num::traits::Pow;
use proconio::*;
use std::f64::consts::PI;

#[allow(non_snake_case)]
pub fn main() {
    input! {
        T: f64,
        (l, x, y) : (f64,f64,f64),
        q: usize,
        ee: [i32;q],
    }

    for e in ee {
        let (y_e, z_e) = decide_yz(e.into(), T, l);

        let base = ((x.pow(2) + (y - y_e).pow(2)) as f64).sqrt();
        let height = z_e.abs();

        let rad = (height / base).atan();
        let angle = 360. * rad / (2. * PI);

        println!("{}", angle);
    }
}

#[allow(non_snake_case)]
fn decide_yz(t: f64, T: f64, l: f64) -> (f64, f64) {
    let rad = t / T * 2. * PI;
    let y = -l / 2. * rad.sin();
    let z = -l / 2. * rad.cos() + l / 2.;

    (y, z)
}
