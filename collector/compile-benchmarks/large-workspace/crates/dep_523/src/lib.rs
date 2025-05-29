pub fn code() {
    println!("Hello from dep_523");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_523");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_523: {t}");
}

pub fn foo() {
    dep_391::code();
    dep_391::code_inlined();
    dep_391::code_generic(1u32);
}
