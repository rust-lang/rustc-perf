pub fn code() {
    println!("Hello from dep_872");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_872");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_872: {t}");
}

pub fn foo() {
    dep_183::code();
    dep_183::code_inlined();
    dep_183::code_generic(1u32);
    dep_316::code();
    dep_316::code_inlined();
    dep_316::code_generic(1u32);
    dep_269::code();
    dep_269::code_inlined();
    dep_269::code_generic(1u32);
    dep_283::code();
    dep_283::code_inlined();
    dep_283::code_generic(1u32);
}
