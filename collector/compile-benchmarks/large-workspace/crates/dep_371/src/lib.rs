pub fn code() {
    println!("Hello from dep_371");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_371");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_371: {t}");
}

pub fn foo() {
    dep_159::code();
    dep_159::code_inlined();
    dep_159::code_generic(1u32);
    dep_75::code();
    dep_75::code_inlined();
    dep_75::code_generic(1u32);
    dep_74::code();
    dep_74::code_inlined();
    dep_74::code_generic(1u32);
}
