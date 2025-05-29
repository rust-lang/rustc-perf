pub fn code() {
    println!("Hello from dep_832");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_832");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_832: {t}");
}

pub fn foo() {
    dep_184::code();
    dep_184::code_inlined();
    dep_184::code_generic(1u32);
    dep_355::code();
    dep_355::code_inlined();
    dep_355::code_generic(1u32);
    dep_275::code();
    dep_275::code_inlined();
    dep_275::code_generic(1u32);
}
