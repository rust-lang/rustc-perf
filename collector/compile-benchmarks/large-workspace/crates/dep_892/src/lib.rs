pub fn code() {
    println!("Hello from dep_892");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_892");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_892: {t}");
}

pub fn foo() {
    dep_234::code();
    dep_234::code_inlined();
    dep_234::code_generic(1u32);
    dep_261::code();
    dep_261::code_inlined();
    dep_261::code_generic(1u32);
    dep_271::code();
    dep_271::code_inlined();
    dep_271::code_generic(1u32);
    dep_341::code();
    dep_341::code_inlined();
    dep_341::code_generic(1u32);
    dep_308::code();
    dep_308::code_inlined();
    dep_308::code_generic(1u32);
}
