pub fn code() {
    println!("Hello from dep_875");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_875");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_875: {t}");
}

pub fn foo() {
    dep_313::code();
    dep_313::code_inlined();
    dep_313::code_generic(1u32);
    dep_299::code();
    dep_299::code_inlined();
    dep_299::code_generic(1u32);
}
