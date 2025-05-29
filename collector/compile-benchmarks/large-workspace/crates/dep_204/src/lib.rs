pub fn code() {
    println!("Hello from dep_204");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_204");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_204: {t}");
}

pub fn foo() {
    dep_155::code();
    dep_155::code_inlined();
    dep_155::code_generic(1u32);
    dep_98::code();
    dep_98::code_inlined();
    dep_98::code_generic(1u32);
    dep_61::code();
    dep_61::code_inlined();
    dep_61::code_generic(1u32);
}
