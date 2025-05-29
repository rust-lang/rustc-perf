pub fn code() {
    println!("Hello from dep_605");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_605");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_605: {t}");
}

pub fn foo() {
    dep_304::code();
    dep_304::code_inlined();
    dep_304::code_generic(1u32);
    dep_384::code();
    dep_384::code_inlined();
    dep_384::code_generic(1u32);
    dep_247::code();
    dep_247::code_inlined();
    dep_247::code_generic(1u32);
}
