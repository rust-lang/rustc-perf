pub fn code() {
    println!("Hello from dep_599");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_599");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_599: {t}");
}

pub fn foo() {
    dep_394::code();
    dep_394::code_inlined();
    dep_394::code_generic(1u32);
    dep_243::code();
    dep_243::code_inlined();
    dep_243::code_generic(1u32);
    dep_285::code();
    dep_285::code_inlined();
    dep_285::code_generic(1u32);
    dep_338::code();
    dep_338::code_inlined();
    dep_338::code_generic(1u32);
    dep_271::code();
    dep_271::code_inlined();
    dep_271::code_generic(1u32);
}
