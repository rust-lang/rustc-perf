pub fn code() {
    println!("Hello from dep_679");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_679");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_679: {t}");
}

pub fn foo() {
    dep_334::code();
    dep_334::code_inlined();
    dep_334::code_generic(1u32);
    dep_256::code();
    dep_256::code_inlined();
    dep_256::code_generic(1u32);
    dep_343::code();
    dep_343::code_inlined();
    dep_343::code_generic(1u32);
    dep_172::code();
    dep_172::code_inlined();
    dep_172::code_generic(1u32);
}
