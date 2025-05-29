pub fn code() {
    println!("Hello from dep_90");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_90");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_90: {t}");
}

pub fn foo() {
    dep_36::code();
    dep_36::code_inlined();
    dep_36::code_generic(1u32);
    dep_56::code();
    dep_56::code_inlined();
    dep_56::code_generic(1u32);
    dep_13::code();
    dep_13::code_inlined();
    dep_13::code_generic(1u32);
    dep_11::code();
    dep_11::code_inlined();
    dep_11::code_generic(1u32);
}
