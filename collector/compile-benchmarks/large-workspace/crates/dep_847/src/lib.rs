pub fn code() {
    println!("Hello from dep_847");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_847");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_847: {t}");
}

pub fn foo() {
    dep_180::code();
    dep_180::code_inlined();
    dep_180::code_generic(1u32);
}
