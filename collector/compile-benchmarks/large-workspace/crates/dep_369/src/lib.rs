pub fn code() {
    println!("Hello from dep_369");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_369");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_369: {t}");
}

pub fn foo() {
    dep_100::code();
    dep_100::code_inlined();
    dep_100::code_generic(1u32);
    dep_89::code();
    dep_89::code_inlined();
    dep_89::code_generic(1u32);
    dep_159::code();
    dep_159::code_inlined();
    dep_159::code_generic(1u32);
    dep_155::code();
    dep_155::code_inlined();
    dep_155::code_generic(1u32);
}
