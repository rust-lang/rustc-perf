pub fn code() {
    println!("Hello from dep_209");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_209");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_209: {t}");
}

pub fn foo() {
    dep_114::code();
    dep_114::code_inlined();
    dep_114::code_generic(1u32);
    dep_98::code();
    dep_98::code_inlined();
    dep_98::code_generic(1u32);
    dep_132::code();
    dep_132::code_inlined();
    dep_132::code_generic(1u32);
}
