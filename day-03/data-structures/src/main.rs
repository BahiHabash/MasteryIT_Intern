#[derive(Debug)]
enum Species { Crab, Octopus, Fish, Clam }

#[derive(Debug)]
enum PoisonType { Acidic, Painful, Lethal }

#[derive(Debug)]
enum Size { Big, Small }

#[derive(Debug)]
enum Weapon {
    Claw(i32, Size),
    Poison(PoisonType),
    None
}

#[derive(Debug)]
struct SeaCreature {
    species: Species,
    name: String,
    arms: i32,
    legs: i32,
    weapon: Weapon,
}

fn main() {
    let start_fish = SeaCreature {
        species: Species::Octopus,
        name: String::from("Star ⭐"),
        arms: 5,
        legs: 0,
        weapon: Weapon::Claw(50, Size::Big),
    };

    match &start_fish.weapon {
        Weapon::Claw(len, size) => match size {
            s @ Size::Big => println!(
                "{} has has weapons & {:?} claw with len {}",
                start_fish.name, s, len
            ),
            s @ Size::Small => println!(
                "{} has has weapons & {:?} claw with len {}",
                start_fish.name, s, len
            ),
        },
        Weapon::Poison(weapon) => match weapon {
            pos @ PoisonType::Acidic => {
                println!("{} has weapons & poison {:?}", start_fish.name, pos)
            }
            pos @ PoisonType::Lethal => {
                println!("{} has weapons & poison {:?}", start_fish.name, pos)
            }
            pos @ PoisonType::Painful => {
                println!("{} has weapons & poison {:?}", start_fish.name, pos)
            }
        },
        Weapon::None => println!("{} has no weapons", start_fish.name),
    }
}
