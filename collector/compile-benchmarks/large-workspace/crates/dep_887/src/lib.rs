pub fn code() {
    println!("Hello from dep_887");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_887");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_887: {t}");
}

pub fn foo() {
    dep_176::code();
    dep_176::code_inlined();
    dep_176::code_generic(1u32);
}
