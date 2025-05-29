pub fn code() {
    println!("Hello from dep_179");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_179");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_179: {t}");
}

pub fn foo() {
    dep_117::code();
    dep_117::code_inlined();
    dep_117::code_generic(1u32);
    dep_69::code();
    dep_69::code_inlined();
    dep_69::code_generic(1u32);
    dep_68::code();
    dep_68::code_inlined();
    dep_68::code_generic(1u32);
}
