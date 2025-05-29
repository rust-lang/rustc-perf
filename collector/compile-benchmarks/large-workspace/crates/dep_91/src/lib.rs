pub fn code() {
    println!("Hello from dep_91");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_91");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_91: {t}");
}

pub fn foo() {
    dep_11::code();
    dep_11::code_inlined();
    dep_11::code_generic(1u32);
    dep_24::code();
    dep_24::code_inlined();
    dep_24::code_generic(1u32);
    dep_52::code();
    dep_52::code_inlined();
    dep_52::code_generic(1u32);
    dep_49::code();
    dep_49::code_inlined();
    dep_49::code_generic(1u32);
    dep_10::code();
    dep_10::code_inlined();
    dep_10::code_generic(1u32);
}
