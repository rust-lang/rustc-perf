pub fn code() {
    println!("Hello from dep_274");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_274");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_274: {t}");
}

pub fn foo() {
    dep_109::code();
    dep_109::code_inlined();
    dep_109::code_generic(1u32);
    dep_98::code();
    dep_98::code_inlined();
    dep_98::code_generic(1u32);
    dep_126::code();
    dep_126::code_inlined();
    dep_126::code_generic(1u32);
    dep_146::code();
    dep_146::code_inlined();
    dep_146::code_generic(1u32);
    dep_104::code();
    dep_104::code_inlined();
    dep_104::code_generic(1u32);
}
