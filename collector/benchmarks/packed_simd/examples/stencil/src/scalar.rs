//! Scalar implementation

pub fn step(
    x0: i32, x1: i32, y0: i32, y1: i32, z0: i32, z1: i32, n_x: i32, n_y: i32,
    _n_z: i32, coef: &[f32; 4], vsq: &[f32], a_in: &[f32], a_out: &mut [f32],
) {
    let n_xy = n_x * n_y;

    for z in z0..z1 {
        for y in y0..y1 {
            for x in x0..x1 {
                let index = (z * n_xy) + (y * n_x) + x;

                macro_rules! a_cur {
                    ($x:expr, $y:expr, $z:expr) => {
                        a_in[(index + $x + $y * n_x + $z * n_xy) as usize]
                    };
                }

                macro_rules! a_next {
                    ($x:expr, $y:expr, $z:expr) => {
                        a_out[(index + $x + $y * n_x + $z * n_xy) as usize]
                    };
                }

                let mut div: f32 = coef[0] * a_cur!(0, 0, 0);
                for i in 1..4 {
                    div += coef[i as usize]
                        * (a_cur!(i, 0, 0)
                            + a_cur!(-i, 0, 0)
                            + a_cur!(0, i, 0)
                            + a_cur!(0, -i, 0)
                            + a_cur!(0, 0, i)
                            + a_cur!(0, 0, -i));
                }
                a_next!(0, 0, 0) = 2. * a_cur!(0, 0, 0) - a_next!(0, 0, 0)
                    + vsq[index as usize] * div;
            }
        }
    }
}

pub fn scalar(
    t0: i32, t1: i32, x0: i32, x1: i32, y0: i32, y1: i32, z0: i32, z1: i32,
    n_x: i32, n_y: i32, n_z: i32, coef: &[f32; 4], vsq: &[f32],
    a_even: &mut [f32], a_odd: &mut [f32],
) {
    for t in t0..t1 {
        if t & 1 == 0 {
            step(
                x0, x1, y0, y1, z0, z1, n_x, n_y, n_z, coef, vsq, a_even,
                a_odd,
            );
        } else {
            step(
                x0, x1, y0, y1, z0, z1, n_x, n_y, n_z, coef, vsq, a_odd,
                a_even,
            );
        }
    }
}

#[cfg(all(test, feature = "ispc"))]
mod tests {
    use super::scalar;
    use crate::ispc_loops::serial;
    use crate::{assert_data_eq, Data};

    #[test]

    fn scalar_ispc_verify() {
        let mut data_scalar = Data::default();
        data_scalar.exec(scalar);

        let mut data_ispc = Data::default();
        data_ispc.exec(serial);

        assert_data_eq(&data_scalar, &data_ispc);
    }
}
