pub fn code() {
    println!("Hello from dep_614");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_614");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_614: {t}");
}

pub fn foo() {
    dep_383::code();
    dep_383::code_inlined();
    dep_383::code_generic(1u32);
    dep_370::code();
    dep_370::code_inlined();
    dep_370::code_generic(1u32);
}
