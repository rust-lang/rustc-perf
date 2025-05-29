pub fn code() {
    println!("Hello from dep_150");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_150");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_150: {t}");
}

pub fn foo() {
    dep_22::code();
    dep_22::code_inlined();
    dep_22::code_generic(1u32);
    dep_39::code();
    dep_39::code_inlined();
    dep_39::code_generic(1u32);
    dep_23::code();
    dep_23::code_inlined();
    dep_23::code_generic(1u32);
    dep_17::code();
    dep_17::code_inlined();
    dep_17::code_generic(1u32);
    dep_45::code();
    dep_45::code_inlined();
    dep_45::code_generic(1u32);
}
