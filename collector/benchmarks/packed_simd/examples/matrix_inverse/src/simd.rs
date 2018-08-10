//! 4x4 matrix inverse using SIMD
use *;

use packed_simd::f32x4;

pub fn inv4x4(m: Matrix4x4) -> Option<Matrix4x4> {
    let m = m.0;
    let m_0 = f32x4::from_slice_aligned(&m[0]);
    let m_1 = f32x4::from_slice_aligned(&m[1]);
    let m_2 = f32x4::from_slice_aligned(&m[2]);
    let m_3 = f32x4::from_slice_aligned(&m[3]);

    let tmp1: f32x4 = shuffle!(m_0, m_1, [0, 1, 4, 5]);
    let row1: f32x4 = shuffle!(m_2, m_3, [0, 1, 4, 5]);

    let row0 = shuffle!(tmp1, row1, [0, 2, 4, 6]);
    let row1: f32x4 = shuffle!(row1, tmp1, [1, 3, 5, 7]);

    let tmp1: f32x4 = shuffle!(m_0, m_1, [2, 3, 6, 7]);
    let row3: f32x4 = shuffle!(m_2, m_3, [2, 3, 6, 7]);
    let row2 = shuffle!(tmp1, row3, [0, 2, 4, 6]);
    let row3 = shuffle!(row3, tmp1, [1, 3, 5, 7]);

    let tmp1: f32x4 = row2 * row3;
    let tmp1 = shuffle!(tmp1, [1, 0, 3, 2]);

    let minor0 = row1 * tmp1;
    let minor1 = row0 * tmp1;
    let tmp1 = shuffle!(tmp1, [2, 3, 0, 1]);
    let minor0 = (row1 * tmp1) - minor0;
    let minor1 = (row0 * tmp1) - minor1;
    let minor1 = shuffle!(minor1, [2, 3, 0, 1]);

    let tmp1 = row1 * row2;
    let tmp1 = shuffle!(tmp1, [1, 0, 3, 2]);
    let minor0 = (row3 * tmp1) + minor0;
    let minor3 = row0 * tmp1;
    let tmp1 = shuffle!(tmp1, [2, 3, 0, 1]);

    let minor0 = minor0 - row3 * tmp1;
    let minor3 = row0 * tmp1 - minor3;
    let minor3 = shuffle!(minor3, [2, 3, 0, 1]);

    let tmp1 = row3 * shuffle!(row1, [2, 3, 0, 1]);
    let tmp1 = shuffle!(tmp1, [1, 0, 3, 2]);
    let row2 = shuffle!(row2, [2, 3, 0, 1]);
    let minor0 = row2 * tmp1 + minor0;
    let minor2 = row0 * tmp1;
    let tmp1 = shuffle!(tmp1, [2, 3, 0, 1]);
    let minor0 = minor0 - row2 * tmp1;
    let minor2 = row0 * tmp1 - minor2;
    let minor2 = shuffle!(minor2, [2, 3, 0, 1]);

    let tmp1 = row0 * row1;
    let tmp1 = shuffle!(tmp1, [1, 0, 3, 2]);
    let minor2 = minor2 + row3 * tmp1;
    let minor3 = row2 * tmp1 - minor3;
    let tmp1 = shuffle!(tmp1, [2, 3, 0, 1]);
    let minor2 = row3 * tmp1 - minor2;
    let minor3 = minor3 - row2 * tmp1;

    let tmp1 = row0 * row3;
    let tmp1 = shuffle!(tmp1, [1, 0, 3, 2]);
    let minor1 = minor1 - row2 * tmp1;
    let minor2 = row1 * tmp1 + minor2;
    let tmp1 = shuffle!(tmp1, [2, 3, 0, 1]);
    let minor1 = row2 * tmp1 + minor1;
    let minor2 = minor2 - row1 * tmp1;

    let tmp1 = row0 * row2;
    let tmp1 = shuffle!(tmp1, [1, 0, 3, 2]);
    let minor1 = row3 * tmp1 + minor1;
    let minor3 = minor3 - row1 * tmp1;
    let tmp1 = shuffle!(tmp1, [2, 3, 0, 1]);
    let minor1 = minor1 - row3 * tmp1;
    let minor3 = row1 * tmp1 + minor3;

    let det = row0 * minor0;
    let det = shuffle!(det, [2, 3, 0, 1]) + det;
    let det = shuffle!(det, [1, 0, 3, 2]) + det;

    if det.sum() == 0. {
        return None;
    }
    let tmp1 = det.recpre();
    let det = tmp1 + tmp1 - det * tmp1 * tmp1;

    let res0 = minor0 * det;
    let res1 = minor1 * det;
    let res2 = minor2 * det;
    let res3 = minor3 * det;

    let mut m = m;

    res0.write_to_slice_aligned(&mut m[0]);
    res1.write_to_slice_aligned(&mut m[1]);
    res2.write_to_slice_aligned(&mut m[2]);
    res3.write_to_slice_aligned(&mut m[3]);

    Some(Matrix4x4(m))
}

#[cfg(test)]
#[test]
fn test() {
    ::test(inv4x4)
}
