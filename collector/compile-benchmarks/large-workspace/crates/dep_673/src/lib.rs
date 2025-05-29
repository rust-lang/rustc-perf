pub fn code() {
    println!("Hello from dep_673");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_673");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_673: {t}");
}

pub fn foo() {
    dep_309::code();
    dep_309::code_inlined();
    dep_309::code_generic(1u32);
    dep_380::code();
    dep_380::code_inlined();
    dep_380::code_generic(1u32);
}
