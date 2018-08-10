//! Scalar mandelbrot implementation

use *;

pub fn mandelbrot(c_x: f64, c_y: f64, max_iter: u32) -> u32 {
    let mut x = c_x;
    let mut y = c_y;
    let mut count = 0;
    while count < max_iter {
        let xx = x * x;
        let yy = y * y;
        let sum = xx + yy;
        if sum > 4.0 {
            break;
        }
        count += 1;
        let xy = x * y;
        x = xx - yy + c_x;
        y = xy * 2.0 + c_y;
    }
    count
}

pub fn output<O: io::Write>(o: &mut O, m: &mut Mandelbrot, limit: u32) {
    let height_step = m.height_step();
    let width_step = m.width_step();
    let out_fn = m.get_format_fn();
    for i in 0..m.height {
        let y = m.top + height_step * i as f64;
        for j in 0..m.width {
            let x = m.left + width_step * j as f64;
            let val = scalar::mandelbrot(x, y, limit);
            out_fn(&mut m.line, j, val);
        }
        o.write(&m.line).unwrap();
    }
}
