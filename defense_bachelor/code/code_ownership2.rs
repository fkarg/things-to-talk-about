fn main() {
    let mut v = vec![];
    v.push("Hello");

    let x = &v[0];
    println!("{}", x);

    v.push("world");

    println!("{}", v[1]);
}
