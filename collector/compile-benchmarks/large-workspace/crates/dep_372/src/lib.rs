pub fn code() {
    println!("Hello from dep_372");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_372");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_372: {t}");
}

pub fn foo() {
    dep_72::code();
    dep_72::code_inlined();
    dep_72::code_generic(1u32);
    dep_68::code();
    dep_68::code_inlined();
    dep_68::code_generic(1u32);
    dep_79::code();
    dep_79::code_inlined();
    dep_79::code_generic(1u32);
    dep_71::code();
    dep_71::code_inlined();
    dep_71::code_generic(1u32);
}
