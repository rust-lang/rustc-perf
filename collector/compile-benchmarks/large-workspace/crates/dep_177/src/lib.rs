pub fn code() {
    println!("Hello from dep_177");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_177");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_177: {t}");
}

pub fn foo() {
    dep_144::code();
    dep_144::code_inlined();
    dep_144::code_generic(1u32);
    dep_107::code();
    dep_107::code_inlined();
    dep_107::code_generic(1u32);
    dep_147::code();
    dep_147::code_inlined();
    dep_147::code_generic(1u32);
    dep_106::code();
    dep_106::code_inlined();
    dep_106::code_generic(1u32);
}
