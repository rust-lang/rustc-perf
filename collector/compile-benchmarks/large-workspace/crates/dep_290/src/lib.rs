pub fn code() {
    println!("Hello from dep_290");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_290");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_290: {t}");
}

pub fn foo() {
    dep_95::code();
    dep_95::code_inlined();
    dep_95::code_generic(1u32);
}
