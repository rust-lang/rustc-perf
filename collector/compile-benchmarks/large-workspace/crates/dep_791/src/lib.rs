pub fn code() {
    println!("Hello from dep_791");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_791");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_791: {t}");
}

pub fn foo() {
    dep_307::code();
    dep_307::code_inlined();
    dep_307::code_generic(1u32);
    dep_265::code();
    dep_265::code_inlined();
    dep_265::code_generic(1u32);
    dep_277::code();
    dep_277::code_inlined();
    dep_277::code_generic(1u32);
}
