pub fn code() {
    println!("Hello from dep_85");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_85");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_85: {t}");
}

pub fn foo() {
    dep_52::code();
    dep_52::code_inlined();
    dep_52::code_generic(1u32);
    dep_14::code();
    dep_14::code_inlined();
    dep_14::code_generic(1u32);
    dep_56::code();
    dep_56::code_inlined();
    dep_56::code_generic(1u32);
    dep_25::code();
    dep_25::code_inlined();
    dep_25::code_generic(1u32);
}
