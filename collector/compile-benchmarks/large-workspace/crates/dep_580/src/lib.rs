pub fn code() {
    println!("Hello from dep_580");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_580");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_580: {t}");
}

pub fn foo() {
    dep_402::code();
    dep_402::code_inlined();
    dep_402::code_generic(1u32);
    dep_234::code();
    dep_234::code_inlined();
    dep_234::code_generic(1u32);
    dep_379::code();
    dep_379::code_inlined();
    dep_379::code_generic(1u32);
}
