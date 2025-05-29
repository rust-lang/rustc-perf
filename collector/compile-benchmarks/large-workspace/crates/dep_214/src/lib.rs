pub fn code() {
    println!("Hello from dep_214");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_214");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_214: {t}");
}

pub fn foo() {
    dep_93::code();
    dep_93::code_inlined();
    dep_93::code_generic(1u32);
    dep_76::code();
    dep_76::code_inlined();
    dep_76::code_generic(1u32);
}
