pub fn code() {
    println!("Hello from dep_195");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_195");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_195: {t}");
}

pub fn foo() {
    dep_132::code();
    dep_132::code_inlined();
    dep_132::code_generic(1u32);
    dep_140::code();
    dep_140::code_inlined();
    dep_140::code_generic(1u32);
    dep_152::code();
    dep_152::code_inlined();
    dep_152::code_generic(1u32);
    dep_83::code();
    dep_83::code_inlined();
    dep_83::code_generic(1u32);
}
