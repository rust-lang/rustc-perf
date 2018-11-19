//! Scalar implementation

pub fn dot_prod(a: &[f32], b: &[f32]) -> f32 {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).map(|v| v.0 * v.1).sum()
}

#[cfg(test)]
#[test]
fn test() {
    crate::test(dot_prod)
}
