pub fn code() {
    println!("Hello from dep_408");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_408");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_408: {t}");
}

pub fn foo() {
    dep_81::code();
    dep_81::code_inlined();
    dep_81::code_generic(1u32);
    dep_136::code();
    dep_136::code_inlined();
    dep_136::code_generic(1u32);
}
