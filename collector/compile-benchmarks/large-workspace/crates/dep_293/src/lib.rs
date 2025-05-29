pub fn code() {
    println!("Hello from dep_293");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_293");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_293: {t}");
}

pub fn foo() {
    dep_101::code();
    dep_101::code_inlined();
    dep_101::code_generic(1u32);
    dep_151::code();
    dep_151::code_inlined();
    dep_151::code_generic(1u32);
    dep_115::code();
    dep_115::code_inlined();
    dep_115::code_generic(1u32);
    dep_147::code();
    dep_147::code_inlined();
    dep_147::code_generic(1u32);
}
