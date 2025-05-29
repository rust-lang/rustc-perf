pub fn code() {
    println!("Hello from dep_723");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_723");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_723: {t}");
}

pub fn foo() {
    dep_305::code();
    dep_305::code_inlined();
    dep_305::code_generic(1u32);
    dep_161::code();
    dep_161::code_inlined();
    dep_161::code_generic(1u32);
    dep_290::code();
    dep_290::code_inlined();
    dep_290::code_generic(1u32);
    dep_329::code();
    dep_329::code_inlined();
    dep_329::code_generic(1u32);
    dep_235::code();
    dep_235::code_inlined();
    dep_235::code_generic(1u32);
}
