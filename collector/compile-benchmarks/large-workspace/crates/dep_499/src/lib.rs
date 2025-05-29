pub fn code() {
    println!("Hello from dep_499");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_499");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_499: {t}");
}

pub fn foo() {
    dep_334::code();
    dep_334::code_inlined();
    dep_334::code_generic(1u32);
    dep_322::code();
    dep_322::code_inlined();
    dep_322::code_generic(1u32);
    dep_170::code();
    dep_170::code_inlined();
    dep_170::code_generic(1u32);
    dep_191::code();
    dep_191::code_inlined();
    dep_191::code_generic(1u32);
    dep_299::code();
    dep_299::code_inlined();
    dep_299::code_generic(1u32);
}
