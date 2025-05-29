pub fn code() {
    println!("Hello from dep_874");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_874");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_874: {t}");
}

pub fn foo() {
    dep_278::code();
    dep_278::code_inlined();
    dep_278::code_generic(1u32);
    dep_404::code();
    dep_404::code_inlined();
    dep_404::code_generic(1u32);
    dep_400::code();
    dep_400::code_inlined();
    dep_400::code_generic(1u32);
    dep_320::code();
    dep_320::code_inlined();
    dep_320::code_generic(1u32);
}
