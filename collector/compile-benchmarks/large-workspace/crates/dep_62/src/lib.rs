pub fn code() {
    println!("Hello from dep_62");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_62");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_62: {t}");
}

pub fn foo() {
    dep_32::code();
    dep_32::code_inlined();
    dep_32::code_generic(1u32);
}
