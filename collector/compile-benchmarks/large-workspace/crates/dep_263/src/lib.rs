pub fn code() {
    println!("Hello from dep_263");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_263");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_263: {t}");
}

pub fn foo() {
    dep_154::code();
    dep_154::code_inlined();
    dep_154::code_generic(1u32);
    dep_108::code();
    dep_108::code_inlined();
    dep_108::code_generic(1u32);
    dep_109::code();
    dep_109::code_inlined();
    dep_109::code_generic(1u32);
}
