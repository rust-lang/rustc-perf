pub fn code() {
    println!("Hello from dep_860");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_860");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_860: {t}");
}

pub fn foo() {
    dep_216::code();
    dep_216::code_inlined();
    dep_216::code_generic(1u32);
    dep_392::code();
    dep_392::code_inlined();
    dep_392::code_generic(1u32);
    dep_278::code();
    dep_278::code_inlined();
    dep_278::code_generic(1u32);
}
