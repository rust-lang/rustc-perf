pub fn code() {
    println!("Hello from dep_452");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_452");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_452: {t}");
}

pub fn foo() {
    dep_243::code();
    dep_243::code_inlined();
    dep_243::code_generic(1u32);
    dep_293::code();
    dep_293::code_inlined();
    dep_293::code_generic(1u32);
    dep_299::code();
    dep_299::code_inlined();
    dep_299::code_generic(1u32);
    dep_286::code();
    dep_286::code_inlined();
    dep_286::code_generic(1u32);
}
