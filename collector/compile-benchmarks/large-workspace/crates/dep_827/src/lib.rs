pub fn code() {
    println!("Hello from dep_827");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_827");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_827: {t}");
}

pub fn foo() {
    dep_212::code();
    dep_212::code_inlined();
    dep_212::code_generic(1u32);
    dep_334::code();
    dep_334::code_inlined();
    dep_334::code_generic(1u32);
    dep_404::code();
    dep_404::code_inlined();
    dep_404::code_generic(1u32);
}
