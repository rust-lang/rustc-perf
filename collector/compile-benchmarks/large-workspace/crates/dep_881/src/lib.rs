pub fn code() {
    println!("Hello from dep_881");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_881");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_881: {t}");
}

pub fn foo() {
    dep_198::code();
    dep_198::code_inlined();
    dep_198::code_generic(1u32);
    dep_397::code();
    dep_397::code_inlined();
    dep_397::code_generic(1u32);
    dep_351::code();
    dep_351::code_inlined();
    dep_351::code_generic(1u32);
}
