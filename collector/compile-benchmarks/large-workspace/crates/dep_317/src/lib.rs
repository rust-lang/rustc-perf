pub fn code() {
    println!("Hello from dep_317");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_317");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_317: {t}");
}

pub fn foo() {
    dep_71::code();
    dep_71::code_inlined();
    dep_71::code_generic(1u32);
    dep_78::code();
    dep_78::code_inlined();
    dep_78::code_generic(1u32);
}
