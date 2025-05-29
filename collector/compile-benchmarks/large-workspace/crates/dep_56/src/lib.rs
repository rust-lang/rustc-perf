pub fn code() {
    println!("Hello from dep_56");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_56");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_56: {t}");
}

pub fn foo() {
    dep_0::code();
    dep_0::code_inlined();
    dep_0::code_generic(1u32);
    dep_7::code();
    dep_7::code_inlined();
    dep_7::code_generic(1u32);
    dep_3::code();
    dep_3::code_inlined();
    dep_3::code_generic(1u32);
}
