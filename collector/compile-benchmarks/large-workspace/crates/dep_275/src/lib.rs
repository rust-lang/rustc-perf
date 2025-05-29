pub fn code() {
    println!("Hello from dep_275");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_275");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_275: {t}");
}

pub fn foo() {
    dep_131::code();
    dep_131::code_inlined();
    dep_131::code_generic(1u32);
    dep_76::code();
    dep_76::code_inlined();
    dep_76::code_generic(1u32);
    dep_136::code();
    dep_136::code_inlined();
    dep_136::code_generic(1u32);
}
