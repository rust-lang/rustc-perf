pub fn code() {
    println!("Hello from dep_786");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_786");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_786: {t}");
}

pub fn foo() {
    dep_276::code();
    dep_276::code_inlined();
    dep_276::code_generic(1u32);
    dep_379::code();
    dep_379::code_inlined();
    dep_379::code_generic(1u32);
    dep_404::code();
    dep_404::code_inlined();
    dep_404::code_generic(1u32);
    dep_350::code();
    dep_350::code_inlined();
    dep_350::code_generic(1u32);
}
