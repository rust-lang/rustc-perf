pub fn code() {
    println!("Hello from dep_123");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_123");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_123: {t}");
}

pub fn foo() {
    dep_52::code();
    dep_52::code_inlined();
    dep_52::code_generic(1u32);
    dep_51::code();
    dep_51::code_inlined();
    dep_51::code_generic(1u32);
    dep_17::code();
    dep_17::code_inlined();
    dep_17::code_generic(1u32);
    dep_23::code();
    dep_23::code_inlined();
    dep_23::code_generic(1u32);
}
