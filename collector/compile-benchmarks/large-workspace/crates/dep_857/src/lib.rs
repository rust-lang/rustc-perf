pub fn code() {
    println!("Hello from dep_857");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_857");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_857: {t}");
}

pub fn foo() {
    dep_349::code();
    dep_349::code_inlined();
    dep_349::code_generic(1u32);
    dep_252::code();
    dep_252::code_inlined();
    dep_252::code_generic(1u32);
    dep_233::code();
    dep_233::code_inlined();
    dep_233::code_generic(1u32);
    dep_192::code();
    dep_192::code_inlined();
    dep_192::code_generic(1u32);
    dep_173::code();
    dep_173::code_inlined();
    dep_173::code_generic(1u32);
}
