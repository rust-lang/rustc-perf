pub fn code() {
    println!("Hello from dep_729");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_729");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_729: {t}");
}

pub fn foo() {
    dep_233::code();
    dep_233::code_inlined();
    dep_233::code_generic(1u32);
}
