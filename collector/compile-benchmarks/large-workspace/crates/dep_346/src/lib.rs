pub fn code() {
    println!("Hello from dep_346");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_346");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_346: {t}");
}

pub fn foo() {
    dep_144::code();
    dep_144::code_inlined();
    dep_144::code_generic(1u32);
    dep_95::code();
    dep_95::code_inlined();
    dep_95::code_generic(1u32);
    dep_103::code();
    dep_103::code_inlined();
    dep_103::code_generic(1u32);
    dep_115::code();
    dep_115::code_inlined();
    dep_115::code_generic(1u32);
    dep_111::code();
    dep_111::code_inlined();
    dep_111::code_generic(1u32);
}
