pub fn code() {
    println!("Hello from dep_759");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_759");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_759: {t}");
}

pub fn foo() {
    dep_167::code();
    dep_167::code_inlined();
    dep_167::code_generic(1u32);
    dep_225::code();
    dep_225::code_inlined();
    dep_225::code_generic(1u32);
    dep_268::code();
    dep_268::code_inlined();
    dep_268::code_generic(1u32);
}
