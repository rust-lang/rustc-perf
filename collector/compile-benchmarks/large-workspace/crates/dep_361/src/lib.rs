pub fn code() {
    println!("Hello from dep_361");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_361");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_361: {t}");
}

pub fn foo() {
    dep_125::code();
    dep_125::code_inlined();
    dep_125::code_generic(1u32);
    dep_98::code();
    dep_98::code_inlined();
    dep_98::code_generic(1u32);
    dep_144::code();
    dep_144::code_inlined();
    dep_144::code_generic(1u32);
}
