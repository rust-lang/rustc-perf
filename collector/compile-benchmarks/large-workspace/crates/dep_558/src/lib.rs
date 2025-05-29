pub fn code() {
    println!("Hello from dep_558");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_558");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_558: {t}");
}

pub fn foo() {
    dep_395::code();
    dep_395::code_inlined();
    dep_395::code_generic(1u32);
    dep_372::code();
    dep_372::code_inlined();
    dep_372::code_generic(1u32);
}
