pub fn code() {
    println!("Hello from dep_602");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_602");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_602: {t}");
}

pub fn foo() {
    dep_304::code();
    dep_304::code_inlined();
    dep_304::code_generic(1u32);
    dep_336::code();
    dep_336::code_inlined();
    dep_336::code_generic(1u32);
    dep_316::code();
    dep_316::code_inlined();
    dep_316::code_generic(1u32);
    dep_209::code();
    dep_209::code_inlined();
    dep_209::code_generic(1u32);
    dep_356::code();
    dep_356::code_inlined();
    dep_356::code_generic(1u32);
}
