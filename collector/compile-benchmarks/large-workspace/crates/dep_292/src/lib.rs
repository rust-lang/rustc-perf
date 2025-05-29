pub fn code() {
    println!("Hello from dep_292");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_292");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_292: {t}");
}

pub fn foo() {
    dep_80::code();
    dep_80::code_inlined();
    dep_80::code_generic(1u32);
    dep_118::code();
    dep_118::code_inlined();
    dep_118::code_generic(1u32);
    dep_98::code();
    dep_98::code_inlined();
    dep_98::code_generic(1u32);
}
