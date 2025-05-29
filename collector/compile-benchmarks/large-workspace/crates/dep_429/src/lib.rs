pub fn code() {
    println!("Hello from dep_429");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_429");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_429: {t}");
}

pub fn foo() {
    dep_393::code();
    dep_393::code_inlined();
    dep_393::code_generic(1u32);
    dep_161::code();
    dep_161::code_inlined();
    dep_161::code_generic(1u32);
    dep_268::code();
    dep_268::code_inlined();
    dep_268::code_generic(1u32);
    dep_262::code();
    dep_262::code_inlined();
    dep_262::code_generic(1u32);
    dep_366::code();
    dep_366::code_inlined();
    dep_366::code_generic(1u32);
}
