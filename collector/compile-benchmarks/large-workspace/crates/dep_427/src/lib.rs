pub fn code() {
    println!("Hello from dep_427");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_427");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_427: {t}");
}

pub fn foo() {
    dep_212::code();
    dep_212::code_inlined();
    dep_212::code_generic(1u32);
    dep_237::code();
    dep_237::code_inlined();
    dep_237::code_generic(1u32);
    dep_401::code();
    dep_401::code_inlined();
    dep_401::code_generic(1u32);
    dep_230::code();
    dep_230::code_inlined();
    dep_230::code_generic(1u32);
}
