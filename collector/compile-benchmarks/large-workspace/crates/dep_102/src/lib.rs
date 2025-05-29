pub fn code() {
    println!("Hello from dep_102");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_102");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_102: {t}");
}

pub fn foo() {
    dep_41::code();
    dep_41::code_inlined();
    dep_41::code_generic(1u32);
    dep_32::code();
    dep_32::code_inlined();
    dep_32::code_generic(1u32);
    dep_10::code();
    dep_10::code_inlined();
    dep_10::code_generic(1u32);
}
