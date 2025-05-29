pub fn code() {
    println!("Hello from dep_778");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_778");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_778: {t}");
}

pub fn foo() {
    dep_404::code();
    dep_404::code_inlined();
    dep_404::code_generic(1u32);
    dep_306::code();
    dep_306::code_inlined();
    dep_306::code_generic(1u32);
    dep_268::code();
    dep_268::code_inlined();
    dep_268::code_generic(1u32);
}
