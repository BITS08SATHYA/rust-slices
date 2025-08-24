fn main() {

    let action_hero = String::from("Sathya Ram");

    let first_name = &action_hero;

    println!("{first_name}");

    let food = "pizza";
    println!("{}", food.len());

    let pizza_slice = &food[0..3];
    println!("{}", pizza_slice.len());

}
