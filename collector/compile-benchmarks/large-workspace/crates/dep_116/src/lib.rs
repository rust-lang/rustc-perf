pub fn code() {
    println!("Hello from dep_116");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_116");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_116: {t}");
}

pub fn foo() {
    dep_46::code();
    dep_46::code_inlined();
    dep_46::code_generic(1u32);
    dep_49::code();
    dep_49::code_inlined();
    dep_49::code_generic(1u32);
}
