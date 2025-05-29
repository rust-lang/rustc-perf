pub fn code() {
    println!("Hello from dep_136");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_136");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_136: {t}");
}

pub fn foo() {
    dep_26::code();
    dep_26::code_inlined();
    dep_26::code_generic(1u32);
    dep_39::code();
    dep_39::code_inlined();
    dep_39::code_generic(1u32);
    dep_16::code();
    dep_16::code_inlined();
    dep_16::code_generic(1u32);
    dep_56::code();
    dep_56::code_inlined();
    dep_56::code_generic(1u32);
}
