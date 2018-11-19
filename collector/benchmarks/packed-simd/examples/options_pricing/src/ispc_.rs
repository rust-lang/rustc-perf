//! Includes the ISPC implementations.

use ispc::*;
ispc_module!(options);

pub mod black_scholes {
    use super::*;

    pub fn serial(
        sa: &[f32], xa: &[f32], ta: &[f32], ra: &[f32], va: &[f32],
        result: &mut [f32], count: usize,
    ) -> f64 {
        unsafe {
            self::options::black_scholes_ispc(
                sa.as_ptr() as *mut f32,
                xa.as_ptr() as *mut f32,
                ta.as_ptr() as *mut f32,
                ra.as_ptr() as *mut f32,
                va.as_ptr() as *mut f32,
                result.as_mut_ptr(),
                count as i32,
            )
        }
    }

    pub fn tasks(
        sa: &[f32], xa: &[f32], ta: &[f32], ra: &[f32], va: &[f32],
        result: &mut [f32], count: usize,
    ) -> f64 {
        unsafe {
            self::options::black_scholes_ispc_tasks(
                sa.as_ptr() as *mut f32,
                xa.as_ptr() as *mut f32,
                ta.as_ptr() as *mut f32,
                ra.as_ptr() as *mut f32,
                va.as_ptr() as *mut f32,
                result.as_mut_ptr(),
                count as i32,
            )
        }
    }
}

pub mod binomial_put {
    use super::*;

    pub fn serial(
        sa: &[f32], xa: &[f32], ta: &[f32], ra: &[f32], va: &[f32],
        result: &mut [f32], count: usize,
    ) -> f64 {
        unsafe {
            self::options::binomial_put_ispc(
                sa.as_ptr() as *mut f32,
                xa.as_ptr() as *mut f32,
                ta.as_ptr() as *mut f32,
                ra.as_ptr() as *mut f32,
                va.as_ptr() as *mut f32,
                result.as_mut_ptr(),
                count as i32,
            )
        }
    }

    pub fn tasks(
        sa: &[f32], xa: &[f32], ta: &[f32], ra: &[f32], va: &[f32],
        result: &mut [f32], count: usize,
    ) -> f64 {
        unsafe {
            self::options::binomial_put_ispc_tasks(
                sa.as_ptr() as *mut f32,
                xa.as_ptr() as *mut f32,
                ta.as_ptr() as *mut f32,
                ra.as_ptr() as *mut f32,
                va.as_ptr() as *mut f32,
                result.as_mut_ptr(),
                count as i32,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn black_scholes() {
        const NOPTS: usize = 1_000_000;
        let mut serial = crate::State::new(NOPTS);
        let mut tasks = crate::State::new(NOPTS);

        let serial_sum = serial.exec(black_scholes::serial);
        let tasks_sum = tasks.exec(black_scholes::tasks);

        assert_eq!(serial, tasks);
        assert_eq!(serial_sum, tasks_sum);
    }

    #[test]
    fn binomial_put() {
        const NOPTS: usize = 1_000_000;
        let mut serial = crate::State::new(NOPTS);
        let mut tasks = crate::State::new(NOPTS);

        let serial_sum = serial.exec(binomial_put::serial);
        let tasks_sum = tasks.exec(binomial_put::tasks);

        assert_eq!(serial, tasks);
        assert_eq!(serial_sum, tasks_sum);
    }
}
