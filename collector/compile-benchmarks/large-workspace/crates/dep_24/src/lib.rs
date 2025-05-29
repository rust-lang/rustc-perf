pub fn code() {
    println!("Hello from dep_24");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_24");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_24: {t}");
}

pub fn foo() {
    dep_3::code();
    dep_3::code_inlined();
    dep_3::code_generic(1u32);
    dep_5::code();
    dep_5::code_inlined();
    dep_5::code_generic(1u32);
    dep_2::code();
    dep_2::code_inlined();
    dep_2::code_generic(1u32);
    dep_8::code();
    dep_8::code_inlined();
    dep_8::code_generic(1u32);
    dep_6::code();
    dep_6::code_inlined();
    dep_6::code_generic(1u32);
}
