#[derive(Debug)]
enum AgeType {
    Child,
    Teen,
    Adult,
    Old,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: Option<AgeType>,
}

fn prefix_name(name: String) -> String {
    format!("JR {}", name)
}

fn postfix_name(age: AgeType) -> AgeType {
    age
}

fn main() {
    let mut p1 = Person { name: String::from("Bahi"), age: Some(AgeType::Adult) };

    // take and return ownership
    p1.name = prefix_name(p1.name);

    // take and return ownership
    if let Some(age) = p1.age.take() {
        p1.age = Some(postfix_name(age));
    }

    println!("Person Is: {:?}", p1);
}
