pub fn code() {
    println!("Hello from dep_350");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_350");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_350: {t}");
}

pub fn foo() {
    dep_79::code();
    dep_79::code_inlined();
    dep_79::code_generic(1u32);
    dep_130::code();
    dep_130::code_inlined();
    dep_130::code_generic(1u32);
    dep_78::code();
    dep_78::code_inlined();
    dep_78::code_generic(1u32);
}
