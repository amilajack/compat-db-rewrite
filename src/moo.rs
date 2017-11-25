#[derive(Debug)]

struct Animal {
    hair: String,
    size: i32
}

fn main() {
    let mut shoo = Animal {
        hair: String::from("asdf"),
        size: 12
    };

    shoo.hair = String::from("nice");
    shoo.size = 33;

    let foo = vec![&shoo, &shoo];

    println!("{:?}", foo);
}
