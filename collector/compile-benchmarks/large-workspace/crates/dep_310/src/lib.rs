pub fn code() {
    println!("Hello from dep_310");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_310");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_310: {t}");
}

pub fn foo() {
    dep_65::code();
    dep_65::code_inlined();
    dep_65::code_generic(1u32);
    dep_62::code();
    dep_62::code_inlined();
    dep_62::code_generic(1u32);
    dep_102::code();
    dep_102::code_inlined();
    dep_102::code_generic(1u32);
    dep_87::code();
    dep_87::code_inlined();
    dep_87::code_generic(1u32);
    dep_156::code();
    dep_156::code_inlined();
    dep_156::code_generic(1u32);
}
