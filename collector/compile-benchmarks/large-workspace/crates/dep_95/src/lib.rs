pub fn code() {
    println!("Hello from dep_95");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_95");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_95: {t}");
}

pub fn foo() {
    dep_15::code();
    dep_15::code_inlined();
    dep_15::code_generic(1u32);
    dep_30::code();
    dep_30::code_inlined();
    dep_30::code_generic(1u32);
    dep_37::code();
    dep_37::code_inlined();
    dep_37::code_generic(1u32);
    dep_49::code();
    dep_49::code_inlined();
    dep_49::code_generic(1u32);
}
