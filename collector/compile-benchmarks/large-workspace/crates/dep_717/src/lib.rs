pub fn code() {
    println!("Hello from dep_717");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_717");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_717: {t}");
}

pub fn foo() {
    dep_176::code();
    dep_176::code_inlined();
    dep_176::code_generic(1u32);
    dep_181::code();
    dep_181::code_inlined();
    dep_181::code_generic(1u32);
    dep_292::code();
    dep_292::code_inlined();
    dep_292::code_generic(1u32);
    dep_374::code();
    dep_374::code_inlined();
    dep_374::code_generic(1u32);
    dep_184::code();
    dep_184::code_inlined();
    dep_184::code_generic(1u32);
}
