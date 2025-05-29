pub fn code() {
    println!("Hello from dep_380");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_380");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_380: {t}");
}

pub fn foo() {
    dep_113::code();
    dep_113::code_inlined();
    dep_113::code_generic(1u32);
}
