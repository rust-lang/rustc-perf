pub fn code() {
    println!("Hello from dep_258");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_258");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_258: {t}");
}

pub fn foo() {
    dep_133::code();
    dep_133::code_inlined();
    dep_133::code_generic(1u32);
    dep_63::code();
    dep_63::code_inlined();
    dep_63::code_generic(1u32);
    dep_112::code();
    dep_112::code_inlined();
    dep_112::code_generic(1u32);
}
