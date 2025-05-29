pub fn code() {
    println!("Hello from dep_336");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_336");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_336: {t}");
}

pub fn foo() {
    dep_130::code();
    dep_130::code_inlined();
    dep_130::code_generic(1u32);
    dep_63::code();
    dep_63::code_inlined();
    dep_63::code_generic(1u32);
}
