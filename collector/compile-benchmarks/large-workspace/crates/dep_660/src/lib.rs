pub fn code() {
    println!("Hello from dep_660");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_660");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_660: {t}");
}

pub fn foo() {
    dep_379::code();
    dep_379::code_inlined();
    dep_379::code_generic(1u32);
    dep_287::code();
    dep_287::code_inlined();
    dep_287::code_generic(1u32);
}
