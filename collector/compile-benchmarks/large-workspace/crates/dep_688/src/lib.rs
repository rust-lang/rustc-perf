pub fn code() {
    println!("Hello from dep_688");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_688");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_688: {t}");
}

pub fn foo() {
    dep_214::code();
    dep_214::code_inlined();
    dep_214::code_generic(1u32);
    dep_390::code();
    dep_390::code_inlined();
    dep_390::code_generic(1u32);
    dep_254::code();
    dep_254::code_inlined();
    dep_254::code_generic(1u32);
}
