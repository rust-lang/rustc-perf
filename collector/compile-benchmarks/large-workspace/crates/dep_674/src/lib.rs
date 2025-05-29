pub fn code() {
    println!("Hello from dep_674");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_674");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_674: {t}");
}

pub fn foo() {
    dep_390::code();
    dep_390::code_inlined();
    dep_390::code_generic(1u32);
    dep_370::code();
    dep_370::code_inlined();
    dep_370::code_generic(1u32);
    dep_161::code();
    dep_161::code_inlined();
    dep_161::code_generic(1u32);
    dep_257::code();
    dep_257::code_inlined();
    dep_257::code_generic(1u32);
}
