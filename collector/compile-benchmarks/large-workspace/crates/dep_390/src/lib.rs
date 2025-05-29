pub fn code() {
    println!("Hello from dep_390");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_390");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_390: {t}");
}

pub fn foo() {
    dep_141::code();
    dep_141::code_inlined();
    dep_141::code_generic(1u32);
    dep_133::code();
    dep_133::code_inlined();
    dep_133::code_generic(1u32);
    dep_102::code();
    dep_102::code_inlined();
    dep_102::code_generic(1u32);
    dep_98::code();
    dep_98::code_inlined();
    dep_98::code_generic(1u32);
}
