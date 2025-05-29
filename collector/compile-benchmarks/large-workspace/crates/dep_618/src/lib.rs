pub fn code() {
    println!("Hello from dep_618");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_618");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_618: {t}");
}

pub fn foo() {
    dep_220::code();
    dep_220::code_inlined();
    dep_220::code_generic(1u32);
    dep_321::code();
    dep_321::code_inlined();
    dep_321::code_generic(1u32);
    dep_238::code();
    dep_238::code_inlined();
    dep_238::code_generic(1u32);
}
