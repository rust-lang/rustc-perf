# Taken from https://github.com/rust-lang/rust/issues/133945

print(
    """#[derive(Debug)]
pub enum Sprites {"""
)
for i in range(10000):
    print(f"    Sprite{i},")
print(
    """}
pub fn main() {
   println!("{:?}", Sprites::Sprite1);
}"""
)
