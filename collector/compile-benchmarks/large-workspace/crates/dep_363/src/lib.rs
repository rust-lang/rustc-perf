pub fn code() {
    println!("Hello from dep_363");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_363");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_363: {t}");
}

pub fn foo() {
    dep_86::code();
    dep_86::code_inlined();
    dep_86::code_generic(1u32);
    dep_118::code();
    dep_118::code_inlined();
    dep_118::code_generic(1u32);
    dep_113::code();
    dep_113::code_inlined();
    dep_113::code_generic(1u32);
    dep_74::code();
    dep_74::code_inlined();
    dep_74::code_generic(1u32);
    dep_146::code();
    dep_146::code_inlined();
    dep_146::code_generic(1u32);
}
