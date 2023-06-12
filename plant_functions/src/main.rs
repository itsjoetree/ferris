fn main() {
    let plant_name: String = String::from("Fern");
    let plant_age_years: u32 = get_plant_age(&plant_name);
    display_plant_info(plant_name, plant_age_years);
}

fn display_plant_info(name: String, age_years: u32) {
    println!("Plant Info: {name} is {age_years} years old");
}

fn get_plant_age(plant_name: &String) -> u32 {
    if plant_name == "Fern" {
        return 10;
    } else if plant_name == "Cactus" {
        return 20;
    }

    0
}