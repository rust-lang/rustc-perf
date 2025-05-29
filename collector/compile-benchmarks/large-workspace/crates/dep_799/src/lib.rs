pub fn code() {
    println!("Hello from dep_799");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_799");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_799: {t}");
}

pub fn foo() {
    dep_265::code();
    dep_265::code_inlined();
    dep_265::code_generic(1u32);
    dep_224::code();
    dep_224::code_inlined();
    dep_224::code_generic(1u32);
    dep_336::code();
    dep_336::code_inlined();
    dep_336::code_generic(1u32);
}
