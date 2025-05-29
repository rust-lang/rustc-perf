pub fn code() {
    println!("Hello from dep_621");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_621");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_621: {t}");
}

pub fn foo() {
    dep_193::code();
    dep_193::code_inlined();
    dep_193::code_generic(1u32);
    dep_178::code();
    dep_178::code_inlined();
    dep_178::code_generic(1u32);
    dep_294::code();
    dep_294::code_inlined();
    dep_294::code_generic(1u32);
    dep_361::code();
    dep_361::code_inlined();
    dep_361::code_generic(1u32);
}
