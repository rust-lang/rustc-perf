pub fn code() {
    println!("Hello from dep_240");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_240");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_240: {t}");
}

pub fn foo() {
    dep_72::code();
    dep_72::code_inlined();
    dep_72::code_generic(1u32);
    dep_151::code();
    dep_151::code_inlined();
    dep_151::code_generic(1u32);
    dep_66::code();
    dep_66::code_inlined();
    dep_66::code_generic(1u32);
    dep_81::code();
    dep_81::code_inlined();
    dep_81::code_generic(1u32);
    dep_107::code();
    dep_107::code_inlined();
    dep_107::code_generic(1u32);
}
