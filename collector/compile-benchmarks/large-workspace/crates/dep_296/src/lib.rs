pub fn code() {
    println!("Hello from dep_296");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_296");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_296: {t}");
}

pub fn foo() {
    dep_107::code();
    dep_107::code_inlined();
    dep_107::code_generic(1u32);
    dep_119::code();
    dep_119::code_inlined();
    dep_119::code_generic(1u32);
    dep_71::code();
    dep_71::code_inlined();
    dep_71::code_generic(1u32);
    dep_78::code();
    dep_78::code_inlined();
    dep_78::code_generic(1u32);
}
