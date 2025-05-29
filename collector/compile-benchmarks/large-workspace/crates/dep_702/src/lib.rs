pub fn code() {
    println!("Hello from dep_702");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_702");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_702: {t}");
}

pub fn foo() {
    dep_380::code();
    dep_380::code_inlined();
    dep_380::code_generic(1u32);
    dep_174::code();
    dep_174::code_inlined();
    dep_174::code_generic(1u32);
    dep_341::code();
    dep_341::code_inlined();
    dep_341::code_generic(1u32);
    dep_198::code();
    dep_198::code_inlined();
    dep_198::code_generic(1u32);
    dep_194::code();
    dep_194::code_inlined();
    dep_194::code_generic(1u32);
}
