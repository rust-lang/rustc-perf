pub fn code() {
    println!("Hello from dep_512");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_512");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_512: {t}");
}

pub fn foo() {
    dep_222::code();
    dep_222::code_inlined();
    dep_222::code_generic(1u32);
}
