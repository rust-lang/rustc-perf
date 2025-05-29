pub fn code() {
    println!("Hello from dep_211");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_211");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_211: {t}");
}

pub fn foo() {
    dep_109::code();
    dep_109::code_inlined();
    dep_109::code_generic(1u32);
    dep_144::code();
    dep_144::code_inlined();
    dep_144::code_generic(1u32);
    dep_151::code();
    dep_151::code_inlined();
    dep_151::code_generic(1u32);
    dep_89::code();
    dep_89::code_inlined();
    dep_89::code_generic(1u32);
    dep_131::code();
    dep_131::code_inlined();
    dep_131::code_generic(1u32);
}
