pub fn code() {
    println!("Hello from dep_115");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_115");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_115: {t}");
}

pub fn foo() {
    dep_10::code();
    dep_10::code_inlined();
    dep_10::code_generic(1u32);
}
