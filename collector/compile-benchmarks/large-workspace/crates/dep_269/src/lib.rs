pub fn code() {
    println!("Hello from dep_269");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_269");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_269: {t}");
}

pub fn foo() {
    dep_158::code();
    dep_158::code_inlined();
    dep_158::code_generic(1u32);
    dep_86::code();
    dep_86::code_inlined();
    dep_86::code_generic(1u32);
    dep_68::code();
    dep_68::code_inlined();
    dep_68::code_generic(1u32);
    dep_143::code();
    dep_143::code_inlined();
    dep_143::code_generic(1u32);
}
