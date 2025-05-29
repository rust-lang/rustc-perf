pub fn code() {
    println!("Hello from dep_906");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_906");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_906: {t}");
}

pub fn foo() {
    dep_370::code();
    dep_370::code_inlined();
    dep_370::code_generic(1u32);
    dep_210::code();
    dep_210::code_inlined();
    dep_210::code_generic(1u32);
    dep_291::code();
    dep_291::code_inlined();
    dep_291::code_generic(1u32);
    dep_248::code();
    dep_248::code_inlined();
    dep_248::code_generic(1u32);
    dep_201::code();
    dep_201::code_inlined();
    dep_201::code_generic(1u32);
}
