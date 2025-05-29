pub fn code() {
    println!("Hello from dep_642");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_642");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_642: {t}");
}

pub fn foo() {
    dep_399::code();
    dep_399::code_inlined();
    dep_399::code_generic(1u32);
    dep_216::code();
    dep_216::code_inlined();
    dep_216::code_generic(1u32);
}
