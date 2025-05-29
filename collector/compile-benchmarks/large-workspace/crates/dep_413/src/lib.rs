pub fn code() {
    println!("Hello from dep_413");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_413");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_413: {t}");
}

pub fn foo() {
    dep_357::code();
    dep_357::code_inlined();
    dep_357::code_generic(1u32);
    dep_300::code();
    dep_300::code_inlined();
    dep_300::code_generic(1u32);
}
