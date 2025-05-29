pub fn code() {
    println!("Hello from dep_899");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_899");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_899: {t}");
}

pub fn foo() {
    dep_269::code();
    dep_269::code_inlined();
    dep_269::code_generic(1u32);
}
