pub fn code() {
    println!("Hello from dep_770");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_770");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_770: {t}");
}

pub fn foo() {
    dep_302::code();
    dep_302::code_inlined();
    dep_302::code_generic(1u32);
    dep_387::code();
    dep_387::code_inlined();
    dep_387::code_generic(1u32);
}
