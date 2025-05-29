pub fn code() {
    println!("Hello from dep_14");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_14");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_14: {t}");
}

pub fn foo() {
    dep_2::code();
    dep_2::code_inlined();
    dep_2::code_generic(1u32);
    dep_6::code();
    dep_6::code_inlined();
    dep_6::code_generic(1u32);
    dep_1::code();
    dep_1::code_inlined();
    dep_1::code_generic(1u32);
    dep_7::code();
    dep_7::code_inlined();
    dep_7::code_generic(1u32);
    dep_8::code();
    dep_8::code_inlined();
    dep_8::code_generic(1u32);
}
