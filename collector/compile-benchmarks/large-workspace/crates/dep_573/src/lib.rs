pub fn code() {
    println!("Hello from dep_573");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_573");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_573: {t}");
}

pub fn foo() {
    dep_222::code();
    dep_222::code_inlined();
    dep_222::code_generic(1u32);
    dep_280::code();
    dep_280::code_inlined();
    dep_280::code_generic(1u32);
    dep_306::code();
    dep_306::code_inlined();
    dep_306::code_generic(1u32);
}
