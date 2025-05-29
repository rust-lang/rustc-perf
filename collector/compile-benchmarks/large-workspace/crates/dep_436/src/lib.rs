pub fn code() {
    println!("Hello from dep_436");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_436");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_436: {t}");
}

pub fn foo() {
    dep_211::code();
    dep_211::code_inlined();
    dep_211::code_generic(1u32);
    dep_281::code();
    dep_281::code_inlined();
    dep_281::code_generic(1u32);
    dep_196::code();
    dep_196::code_inlined();
    dep_196::code_generic(1u32);
    dep_172::code();
    dep_172::code_inlined();
    dep_172::code_generic(1u32);
    dep_243::code();
    dep_243::code_inlined();
    dep_243::code_generic(1u32);
}
