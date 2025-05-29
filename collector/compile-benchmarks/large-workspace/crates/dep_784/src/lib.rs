pub fn code() {
    println!("Hello from dep_784");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_784");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_784: {t}");
}

pub fn foo() {
    dep_212::code();
    dep_212::code_inlined();
    dep_212::code_generic(1u32);
    dep_274::code();
    dep_274::code_inlined();
    dep_274::code_generic(1u32);
    dep_358::code();
    dep_358::code_inlined();
    dep_358::code_generic(1u32);
    dep_401::code();
    dep_401::code_inlined();
    dep_401::code_generic(1u32);
}
