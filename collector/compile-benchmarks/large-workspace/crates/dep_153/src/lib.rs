pub fn code() {
    println!("Hello from dep_153");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_153");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_153: {t}");
}

pub fn foo() {
    dep_35::code();
    dep_35::code_inlined();
    dep_35::code_generic(1u32);
    dep_31::code();
    dep_31::code_inlined();
    dep_31::code_generic(1u32);
}
