pub fn code() {
    println!("Hello from dep_553");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_553");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_553: {t}");
}

pub fn foo() {
    dep_338::code();
    dep_338::code_inlined();
    dep_338::code_generic(1u32);
    dep_347::code();
    dep_347::code_inlined();
    dep_347::code_generic(1u32);
    dep_380::code();
    dep_380::code_inlined();
    dep_380::code_generic(1u32);
    dep_172::code();
    dep_172::code_inlined();
    dep_172::code_generic(1u32);
}
