pub fn code() {
    println!("Hello from dep_254");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_254");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_254: {t}");
}

pub fn foo() {
    dep_74::code();
    dep_74::code_inlined();
    dep_74::code_generic(1u32);
}
