pub fn code() {
    println!("Hello from dep_394");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_394");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_394: {t}");
}

pub fn foo() {
    dep_150::code();
    dep_150::code_inlined();
    dep_150::code_generic(1u32);
    dep_89::code();
    dep_89::code_inlined();
    dep_89::code_generic(1u32);
    dep_79::code();
    dep_79::code_inlined();
    dep_79::code_generic(1u32);
    dep_93::code();
    dep_93::code_inlined();
    dep_93::code_generic(1u32);
}
