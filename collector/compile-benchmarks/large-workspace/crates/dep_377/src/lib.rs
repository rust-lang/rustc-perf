pub fn code() {
    println!("Hello from dep_377");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_377");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_377: {t}");
}

pub fn foo() {
    dep_127::code();
    dep_127::code_inlined();
    dep_127::code_generic(1u32);
    dep_144::code();
    dep_144::code_inlined();
    dep_144::code_generic(1u32);
    dep_86::code();
    dep_86::code_inlined();
    dep_86::code_generic(1u32);
    dep_128::code();
    dep_128::code_inlined();
    dep_128::code_generic(1u32);
    dep_149::code();
    dep_149::code_inlined();
    dep_149::code_generic(1u32);
}
