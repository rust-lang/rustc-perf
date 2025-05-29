pub fn code() {
    println!("Hello from dep_495");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_495");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_495: {t}");
}

pub fn foo() {
    dep_245::code();
    dep_245::code_inlined();
    dep_245::code_generic(1u32);
    dep_283::code();
    dep_283::code_inlined();
    dep_283::code_generic(1u32);
    dep_350::code();
    dep_350::code_inlined();
    dep_350::code_generic(1u32);
    dep_284::code();
    dep_284::code_inlined();
    dep_284::code_generic(1u32);
    dep_318::code();
    dep_318::code_inlined();
    dep_318::code_generic(1u32);
}
