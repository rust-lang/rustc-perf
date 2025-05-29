pub fn code() {
    println!("Hello from dep_805");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_805");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_805: {t}");
}

pub fn foo() {
    dep_343::code();
    dep_343::code_inlined();
    dep_343::code_generic(1u32);
}
