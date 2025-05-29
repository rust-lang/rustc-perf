pub fn code() {
    println!("Hello from dep_671");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_671");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_671: {t}");
}

pub fn foo() {
    dep_362::code();
    dep_362::code_inlined();
    dep_362::code_generic(1u32);
    dep_285::code();
    dep_285::code_inlined();
    dep_285::code_generic(1u32);
    dep_399::code();
    dep_399::code_inlined();
    dep_399::code_generic(1u32);
    dep_231::code();
    dep_231::code_inlined();
    dep_231::code_generic(1u32);
    dep_338::code();
    dep_338::code_inlined();
    dep_338::code_generic(1u32);
}
