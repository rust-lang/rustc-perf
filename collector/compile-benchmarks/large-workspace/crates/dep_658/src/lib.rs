pub fn code() {
    println!("Hello from dep_658");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_658");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_658: {t}");
}

pub fn foo() {
    dep_209::code();
    dep_209::code_inlined();
    dep_209::code_generic(1u32);
    dep_283::code();
    dep_283::code_inlined();
    dep_283::code_generic(1u32);
}
