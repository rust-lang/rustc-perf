pub fn code() {
    println!("Hello from dep_118");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_118");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_118: {t}");
}

pub fn foo() {
    dep_52::code();
    dep_52::code_inlined();
    dep_52::code_generic(1u32);
    dep_39::code();
    dep_39::code_inlined();
    dep_39::code_generic(1u32);
    dep_45::code();
    dep_45::code_inlined();
    dep_45::code_generic(1u32);
    dep_11::code();
    dep_11::code_inlined();
    dep_11::code_generic(1u32);
    dep_22::code();
    dep_22::code_inlined();
    dep_22::code_generic(1u32);
}
