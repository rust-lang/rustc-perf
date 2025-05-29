pub fn code() {
    println!("Hello from dep_206");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_206");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_206: {t}");
}

pub fn foo() {
    dep_99::code();
    dep_99::code_inlined();
    dep_99::code_generic(1u32);
    dep_103::code();
    dep_103::code_inlined();
    dep_103::code_generic(1u32);
}
