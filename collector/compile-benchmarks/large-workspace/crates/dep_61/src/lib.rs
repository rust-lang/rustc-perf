pub fn code() {
    println!("Hello from dep_61");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_61");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_61: {t}");
}

pub fn foo() {
    dep_33::code();
    dep_33::code_inlined();
    dep_33::code_generic(1u32);
    dep_18::code();
    dep_18::code_inlined();
    dep_18::code_generic(1u32);
    dep_11::code();
    dep_11::code_inlined();
    dep_11::code_generic(1u32);
    dep_44::code();
    dep_44::code_inlined();
    dep_44::code_generic(1u32);
}
