// The Computer Language Benchmarks Game
// https://benchmarksgame-team.pages.debian.net
//
// contributed by the Rust Project Developers
// contributed by TeXitoi

use std::f64::consts::PI;
const SOLAR_MASS: f64 = 4.0 * PI * PI;
const DAYS_PER_YEAR: f64 = 365.24;

struct Body {
    x: [f64; 3],
    v: [f64; 3],
    mass: f64,
}

const N_BODIES: usize = 5;
#[cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
const BODIES: [Body; N_BODIES] = [
    // Sun
    Body { x: [0., 0., 0.], v: [0., 0., 0.], mass: SOLAR_MASS },
    // Jupiter
    Body {
        x: [
            4.84143144246472090e+00,
            -1.16032004402742839e+00,
            -1.03622044471123109e-01,
        ],
        v: [
            1.66007664274403694e-03 * DAYS_PER_YEAR,
            7.69901118419740425e-03 * DAYS_PER_YEAR,
            -6.90460016972063023e-05 * DAYS_PER_YEAR,
        ],
        mass: 9.54791938424326609e-04 * SOLAR_MASS,
    },
    // Saturn
    Body {
        x: [
            8.34336671824457987e+00,
            4.12479856412430479e+00,
            -4.03523417114321381e-01,
        ],
        v: [
            -2.76742510726862411e-03 * DAYS_PER_YEAR,
            4.99852801234917238e-03 * DAYS_PER_YEAR,
            2.30417297573763929e-05 * DAYS_PER_YEAR,
        ],
        mass: 2.85885980666130812e-04 * SOLAR_MASS,
    },
    // Uranus
    Body {
        x: [
            1.28943695621391310e+01,
            -1.51111514016986312e+01,
            -2.23307578892655734e-01,
        ],
        v: [
            2.96460137564761618e-03 * DAYS_PER_YEAR,
            2.37847173959480950e-03 * DAYS_PER_YEAR,
            -2.96589568540237556e-05 * DAYS_PER_YEAR,
        ],
        mass: 4.36624404335156298e-05 * SOLAR_MASS,
    },
    // Neptune
    Body {
        x: [
            1.53796971148509165e+01,
            -2.59193146099879641e+01,
            1.79258772950371181e-01,
        ],
        v: [
            2.68067772490389322e-03 * DAYS_PER_YEAR,
            1.62824170038242295e-03 * DAYS_PER_YEAR,
            -9.51592254519715870e-05 * DAYS_PER_YEAR,
        ],
        mass: 5.15138902046611451e-05 * SOLAR_MASS,
    },
];

fn advance(bodies: &mut [Body; N_BODIES], dt: f64) {
    let mut b_slice: &mut [_] = bodies;
    while let Some(bi) = shift_mut_ref(&mut b_slice) {
        for bj in b_slice.iter_mut() {
            let mut dx = [0.; 3];
            for (dx, (x_i, x_j)) in
                dx.iter_mut().zip(bi.x.iter().zip(bj.x.iter()))
            {
                *dx = x_i - x_j;
            }

            let mut d2: f64 = 0.;
            for dx in &dx {
                d2 += dx * dx;
            }
            let mag = dt / (d2 * d2.sqrt());

            let massi_mag = bi.mass * mag;
            let massj_mag = bj.mass * mag;
            for (v_j, (v_i, dx)) in
                bj.v.iter_mut().zip(bi.v.iter_mut().zip(dx.iter()))
            {
                *v_j += dx * massi_mag;
                *v_i -= dx * massj_mag;
            }
        }
        for (x, v) in bi.x.iter_mut().zip(bi.v.iter()) {
            *x += dt * v;
        }
    }
}

fn energy(bodies: &[Body; N_BODIES]) -> f64 {
    let mut e = 0.0;
    let mut bodies = bodies.iter();
    while let Some(bi) = bodies.next() {
        let mut e_l = 0.;
        for v in &bi.v {
            e_l += v * v;
        }
        e += e_l * bi.mass / 2.0;
        for bj in bodies.clone() {
            let mut dist = 0.;
            for (xi, xj) in bi.x.iter().zip(bj.x.iter()) {
                let dx = xi - xj;
                dist += dx * dx;
            }
            e -= bi.mass * bj.mass / dist.sqrt();
        }
    }
    e
}

fn offset_momentum(bodies: &mut [Body; N_BODIES]) {
    let mut p = [0.; 3];
    for bi in bodies.iter() {
        for (p, v) in p.iter_mut().zip(bi.v.iter()) {
            *p += v * bi.mass;
        }
    }
    let sun = &mut bodies[0];
    for (v, p) in sun.v.iter_mut().zip(p.iter()) {
        *v = -p / SOLAR_MASS;
    }
}

/// Pop a mutable reference off the head of a slice, mutating the slice to no
/// longer contain the mutable reference.
#[cfg_attr(feature = "cargo-clippy", allow(clippy::mut_mut))]
fn shift_mut_ref<'a, T>(r: &mut &'a mut [T]) -> Option<&'a mut T> {
    if r.is_empty() {
        return None;
    }
    let tmp = std::mem::replace(r, &mut []);
    let (h, t) = tmp.split_at_mut(1);
    *r = t;
    Some(&mut h[0])
}

pub fn run(n: usize) -> (f64, f64) {
    let mut bodies = BODIES;
    offset_momentum(&mut bodies);
    let a = energy(&bodies);
    for _ in 0..n {
        advance(&mut bodies, 0.01);
    }
    let b = energy(&bodies);
    (a, b)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        for &(size, a_e, b_e) in crate::RESULTS {
            let (a, b) = super::run(size);
            assert_eq!(format!("{:.9}", a), a_e);
            assert_eq!(format!("{:.9}", b), b_e);
        }
    }
}
