pub fn code() {
    println!("Hello from dep_565");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_565");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_565: {t}");
}

pub fn foo() {
    dep_373::code();
    dep_373::code_inlined();
    dep_373::code_generic(1u32);
    dep_353::code();
    dep_353::code_inlined();
    dep_353::code_generic(1u32);
    dep_302::code();
    dep_302::code_inlined();
    dep_302::code_generic(1u32);
}
