pub fn code() {
    println!("Hello from dep_540");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_540");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_540: {t}");
}

pub fn foo() {
    dep_250::code();
    dep_250::code_inlined();
    dep_250::code_generic(1u32);
    dep_382::code();
    dep_382::code_inlined();
    dep_382::code_generic(1u32);
    dep_230::code();
    dep_230::code_inlined();
    dep_230::code_generic(1u32);
}
