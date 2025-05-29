pub fn code() {
    println!("Hello from dep_81");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_81");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_81: {t}");
}

pub fn foo() {
    dep_27::code();
    dep_27::code_inlined();
    dep_27::code_generic(1u32);
    dep_10::code();
    dep_10::code_inlined();
    dep_10::code_generic(1u32);
    dep_32::code();
    dep_32::code_inlined();
    dep_32::code_generic(1u32);
    dep_11::code();
    dep_11::code_inlined();
    dep_11::code_generic(1u32);
}
