pub fn code() {
    println!("Hello from dep_326");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_326");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_326: {t}");
}

pub fn foo() {
    dep_156::code();
    dep_156::code_inlined();
    dep_156::code_generic(1u32);
    dep_68::code();
    dep_68::code_inlined();
    dep_68::code_generic(1u32);
    dep_140::code();
    dep_140::code_inlined();
    dep_140::code_generic(1u32);
    dep_141::code();
    dep_141::code_inlined();
    dep_141::code_generic(1u32);
}
