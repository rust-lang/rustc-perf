pub fn code() {
    println!("Hello from dep_714");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_714");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_714: {t}");
}

pub fn foo() {
    dep_166::code();
    dep_166::code_inlined();
    dep_166::code_generic(1u32);
    dep_374::code();
    dep_374::code_inlined();
    dep_374::code_generic(1u32);
}
