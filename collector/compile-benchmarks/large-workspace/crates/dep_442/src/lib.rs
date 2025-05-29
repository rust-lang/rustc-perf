pub fn code() {
    println!("Hello from dep_442");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_442");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_442: {t}");
}

pub fn foo() {
    dep_329::code();
    dep_329::code_inlined();
    dep_329::code_generic(1u32);
    dep_280::code();
    dep_280::code_inlined();
    dep_280::code_generic(1u32);
    dep_166::code();
    dep_166::code_inlined();
    dep_166::code_generic(1u32);
    dep_161::code();
    dep_161::code_inlined();
    dep_161::code_generic(1u32);
}
