pub fn code() {
    println!("Hello from dep_813");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_813");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_813: {t}");
}

pub fn foo() {
    dep_298::code();
    dep_298::code_inlined();
    dep_298::code_generic(1u32);
    dep_376::code();
    dep_376::code_inlined();
    dep_376::code_generic(1u32);
    dep_269::code();
    dep_269::code_inlined();
    dep_269::code_generic(1u32);
}
