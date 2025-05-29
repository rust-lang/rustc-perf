pub fn code() {
    println!("Hello from dep_139");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_139");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_139: {t}");
}

pub fn foo() {
    dep_37::code();
    dep_37::code_inlined();
    dep_37::code_generic(1u32);
    dep_58::code();
    dep_58::code_inlined();
    dep_58::code_generic(1u32);
}
