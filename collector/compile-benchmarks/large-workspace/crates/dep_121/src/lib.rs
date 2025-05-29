pub fn code() {
    println!("Hello from dep_121");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_121");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_121: {t}");
}

pub fn foo() {
    dep_23::code();
    dep_23::code_inlined();
    dep_23::code_generic(1u32);
    dep_46::code();
    dep_46::code_inlined();
    dep_46::code_generic(1u32);
}
