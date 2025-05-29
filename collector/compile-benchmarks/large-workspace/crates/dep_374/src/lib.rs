pub fn code() {
    println!("Hello from dep_374");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_374");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_374: {t}");
}

pub fn foo() {
    dep_159::code();
    dep_159::code_inlined();
    dep_159::code_generic(1u32);
}
