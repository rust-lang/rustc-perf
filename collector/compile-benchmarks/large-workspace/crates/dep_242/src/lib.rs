pub fn code() {
    println!("Hello from dep_242");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_242");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_242: {t}");
}

pub fn foo() {
    dep_116::code();
    dep_116::code_inlined();
    dep_116::code_generic(1u32);
    dep_107::code();
    dep_107::code_inlined();
    dep_107::code_generic(1u32);
    dep_147::code();
    dep_147::code_inlined();
    dep_147::code_generic(1u32);
}
