pub fn code() {
    println!("Hello from dep_734");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_734");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_734: {t}");
}

pub fn foo() {
    dep_193::code();
    dep_193::code_inlined();
    dep_193::code_generic(1u32);
    dep_200::code();
    dep_200::code_inlined();
    dep_200::code_generic(1u32);
    dep_321::code();
    dep_321::code_inlined();
    dep_321::code_generic(1u32);
}
