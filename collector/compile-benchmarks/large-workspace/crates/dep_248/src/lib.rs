pub fn code() {
    println!("Hello from dep_248");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_248");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_248: {t}");
}

pub fn foo() {
    dep_62::code();
    dep_62::code_inlined();
    dep_62::code_generic(1u32);
    dep_122::code();
    dep_122::code_inlined();
    dep_122::code_generic(1u32);
    dep_147::code();
    dep_147::code_inlined();
    dep_147::code_generic(1u32);
    dep_86::code();
    dep_86::code_inlined();
    dep_86::code_generic(1u32);
}
