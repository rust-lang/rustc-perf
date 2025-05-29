pub fn code() {
    println!("Hello from dep_339");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_339");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_339: {t}");
}

pub fn foo() {
    dep_156::code();
    dep_156::code_inlined();
    dep_156::code_generic(1u32);
    dep_146::code();
    dep_146::code_inlined();
    dep_146::code_generic(1u32);
    dep_94::code();
    dep_94::code_inlined();
    dep_94::code_generic(1u32);
    dep_130::code();
    dep_130::code_inlined();
    dep_130::code_generic(1u32);
}
