pub fn code() {
    println!("Hello from dep_168");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_168");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_168: {t}");
}

pub fn foo() {
    dep_85::code();
    dep_85::code_inlined();
    dep_85::code_generic(1u32);
    dep_93::code();
    dep_93::code_inlined();
    dep_93::code_generic(1u32);
}
