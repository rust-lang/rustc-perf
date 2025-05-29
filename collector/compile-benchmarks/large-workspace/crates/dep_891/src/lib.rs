pub fn code() {
    println!("Hello from dep_891");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_891");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_891: {t}");
}

pub fn foo() {
    dep_269::code();
    dep_269::code_inlined();
    dep_269::code_generic(1u32);
    dep_402::code();
    dep_402::code_inlined();
    dep_402::code_generic(1u32);
    dep_170::code();
    dep_170::code_inlined();
    dep_170::code_generic(1u32);
}
