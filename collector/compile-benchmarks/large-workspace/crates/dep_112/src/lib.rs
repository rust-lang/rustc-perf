pub fn code() {
    println!("Hello from dep_112");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_112");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_112: {t}");
}

pub fn foo() {
    dep_38::code();
    dep_38::code_inlined();
    dep_38::code_generic(1u32);
    dep_32::code();
    dep_32::code_inlined();
    dep_32::code_generic(1u32);
    dep_26::code();
    dep_26::code_inlined();
    dep_26::code_generic(1u32);
    dep_44::code();
    dep_44::code_inlined();
    dep_44::code_generic(1u32);
}
