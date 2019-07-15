fn main() {
    let mut v = vec![];     // ---|
    v.push("Hello");        // <--|
                            //    |
    let x = &v[0];          // -| |
                            //  | |
                            //  | |
    v.push("world");        // <X-|
    println!("{}", x);      // -| |
}                           // ---|
