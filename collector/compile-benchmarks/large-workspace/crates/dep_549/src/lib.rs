pub fn code() {
    println!("Hello from dep_549");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_549");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_549: {t}");
}

pub fn foo() {
    dep_168::code();
    dep_168::code_inlined();
    dep_168::code_generic(1u32);
}
