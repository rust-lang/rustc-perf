pub fn code() {
    println!("Hello from dep_876");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_876");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_876: {t}");
}

pub fn foo() {
    dep_352::code();
    dep_352::code_inlined();
    dep_352::code_generic(1u32);
    dep_356::code();
    dep_356::code_inlined();
    dep_356::code_generic(1u32);
    dep_293::code();
    dep_293::code_inlined();
    dep_293::code_generic(1u32);
}
