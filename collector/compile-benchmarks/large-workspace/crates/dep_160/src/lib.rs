pub fn code() {
    println!("Hello from dep_160");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_160");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_160: {t}");
}

pub fn foo() {
    dep_60::code();
    dep_60::code_inlined();
    dep_60::code_generic(1u32);
    dep_119::code();
    dep_119::code_inlined();
    dep_119::code_generic(1u32);
    dep_62::code();
    dep_62::code_inlined();
    dep_62::code_generic(1u32);
    dep_74::code();
    dep_74::code_inlined();
    dep_74::code_generic(1u32);
}
