pub fn code() {
    println!("Hello from dep_870");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_870");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_870: {t}");
}

pub fn foo() {
    dep_271::code();
    dep_271::code_inlined();
    dep_271::code_generic(1u32);
    dep_207::code();
    dep_207::code_inlined();
    dep_207::code_generic(1u32);
    dep_221::code();
    dep_221::code_inlined();
    dep_221::code_generic(1u32);
    dep_223::code();
    dep_223::code_inlined();
    dep_223::code_generic(1u32);
    dep_320::code();
    dep_320::code_inlined();
    dep_320::code_generic(1u32);
}
