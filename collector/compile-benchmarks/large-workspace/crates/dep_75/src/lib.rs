pub fn code() {
    println!("Hello from dep_75");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_75");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_75: {t}");
}

pub fn foo() {
    dep_48::code();
    dep_48::code_inlined();
    dep_48::code_generic(1u32);
    dep_28::code();
    dep_28::code_inlined();
    dep_28::code_generic(1u32);
    dep_14::code();
    dep_14::code_inlined();
    dep_14::code_generic(1u32);
}
