pub fn code() {
    println!("Hello from dep_176");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_176");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_176: {t}");
}

pub fn foo() {
    dep_107::code();
    dep_107::code_inlined();
    dep_107::code_generic(1u32);
    dep_132::code();
    dep_132::code_inlined();
    dep_132::code_generic(1u32);
}
