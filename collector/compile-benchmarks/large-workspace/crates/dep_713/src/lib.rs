pub fn code() {
    println!("Hello from dep_713");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_713");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_713: {t}");
}

pub fn foo() {
    dep_312::code();
    dep_312::code_inlined();
    dep_312::code_generic(1u32);
    dep_269::code();
    dep_269::code_inlined();
    dep_269::code_generic(1u32);
}
