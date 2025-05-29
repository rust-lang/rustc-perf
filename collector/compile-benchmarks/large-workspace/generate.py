"""
This script can be used to regenerate the `large-workspace` benchmark
using a custom number of dependent crates.
"""
import random
from pathlib import Path
from typing import List, Literal


def call_crate(name: str) -> str:
    return f"""    {name}::code();
    {name}::code_inlined();
    {name}::code_generic(1u32);
"""


def generate_crate(dir: Path, name: str, kind: Literal["lib", "bin"], deps: List[str]):
    if kind == "lib":
        root_file = dir / "src" / "lib.rs"
        root_fn = "foo"
    else:
        root_file = dir / "src" / "main.rs"
        root_fn = "main"

    root_file.parent.mkdir(exist_ok=True, parents=True)

    dep_prefix = "crates" if kind == "bin" else ".."

    dep_entries = [f"""{dep} = {{ path = "{dep_prefix}/{dep}" }}""" for dep in deps]
    workspace = "\n\n[workspace]" if kind == "bin" else ""
    with open(dir / "Cargo.toml", "w") as f:
        f.write(f"""[package]
name = "{name}"
version = "0.1.0"
authors = ["rustc-perf developers"]
edition = "2024"

[dependencies]
{'\n'.join(dep_entries)}{workspace}
""")
    with open(root_file, "w") as f:
        if kind == "lib":
            f.write(f"""pub fn code() {{
    println!("Hello from {name}");
}}

#[inline(always)]
pub fn code_inlined() {{
    println!("Hello from {name}");
}}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {{
    println!("Hello from {name}: {{t}}");
}}

""")
        if len(deps) > 0:
            vis = "pub " if kind == "lib" else ""
            f.write(f"{vis}fn {root_fn}() {{\n")
            for dep in deps:
                f.write(call_crate(dep))
            f.write("}\n")


if __name__ == "__main__":
    random.seed(42)

    Path("crates").mkdir(exist_ok=True)

    # Counts of crates at each "level" of the hierarchy
    # Crates in level N directly depend on crates in level N-1
    CRATE_LEVEL_COUNTS = [
        10,
        50,
        100,
        250,
        500
    ]

    crate_levels = []

    crate_id_counter = 0
    for (level_index, crate_count) in enumerate(CRATE_LEVEL_COUNTS):
        level = []

        for _ in range(crate_count):
            crate_id = crate_id_counter
            crate_id_counter += 1
            name = f"dep_{crate_id}"
            level.append(name)

            deps = []
            if level_index > 0:
                prev_level = list(crate_levels[level_index - 1])
                random.shuffle(prev_level)
                # Depend on 1-5 previous crates
                deps = prev_level[:random.randint(1, 5)]
            generate_crate(Path(f"crates/{name}"), name, "lib", deps)
        crate_levels.append(level)

    generate_crate(Path("."), "large-workspace", "bin", crate_levels[-1])
