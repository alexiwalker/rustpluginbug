use inner_crate;

fn main() {
    let foo = inner_crate::SomeStruct::new(1,2,3);
    let sum = foo.sum();
    dbg!(&sum);
}
