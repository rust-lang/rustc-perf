pub fn code() {
    println!("Hello from dep_426");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_426");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_426: {t}");
}

pub fn foo() {
    dep_321::code();
    dep_321::code_inlined();
    dep_321::code_generic(1u32);
}
