fn main() {

    let action_hero = String::from("Sathya Ram");

    let first_name = &action_hero[..6];
    
    let last_name = &action_hero[7..];

    println!("{first_name}");
    
    println!("{last_name}");

    let food = "pizza";
    println!("{}", food.len());

    let pizza_slice = &food[0..3];
    println!("{}", pizza_slice.len());

}
