pub fn code() {
    println!("Hello from dep_307");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_307");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_307: {t}");
}

pub fn foo() {
    dep_113::code();
    dep_113::code_inlined();
    dep_113::code_generic(1u32);
    dep_142::code();
    dep_142::code_inlined();
    dep_142::code_generic(1u32);
}
