pub fn code() {
    println!("Hello from dep_437");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_437");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_437: {t}");
}

pub fn foo() {
    dep_379::code();
    dep_379::code_inlined();
    dep_379::code_generic(1u32);
    dep_282::code();
    dep_282::code_inlined();
    dep_282::code_generic(1u32);
    dep_344::code();
    dep_344::code_inlined();
    dep_344::code_generic(1u32);
}
