pub fn code() {
    println!("Hello from dep_623");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_623");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_623: {t}");
}

pub fn foo() {
    dep_256::code();
    dep_256::code_inlined();
    dep_256::code_generic(1u32);
    dep_291::code();
    dep_291::code_inlined();
    dep_291::code_generic(1u32);
    dep_179::code();
    dep_179::code_inlined();
    dep_179::code_generic(1u32);
    dep_246::code();
    dep_246::code_inlined();
    dep_246::code_generic(1u32);
    dep_339::code();
    dep_339::code_inlined();
    dep_339::code_generic(1u32);
}
