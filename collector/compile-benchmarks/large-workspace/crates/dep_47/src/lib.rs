pub fn code() {
    println!("Hello from dep_47");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_47");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_47: {t}");
}

pub fn foo() {
    dep_2::code();
    dep_2::code_inlined();
    dep_2::code_generic(1u32);
    dep_3::code();
    dep_3::code_inlined();
    dep_3::code_generic(1u32);
    dep_4::code();
    dep_4::code_inlined();
    dep_4::code_generic(1u32);
    dep_5::code();
    dep_5::code_inlined();
    dep_5::code_generic(1u32);
}
