pub fn code() {
    println!("Hello from dep_845");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_845");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_845: {t}");
}

pub fn foo() {
    dep_227::code();
    dep_227::code_inlined();
    dep_227::code_generic(1u32);
    dep_293::code();
    dep_293::code_inlined();
    dep_293::code_generic(1u32);
}
