use std::os::unix::fs::lchown;

fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day!");
}

fn main() {

    let action_hero = String::from("Sathya Ram");
    
    do_hero_stuff(&action_hero);
    
    let another_hero = "Vinothini";
    do_hero_stuff(&another_hero);
    // 
    // let first_name = &action_hero[..6];
    // 
    // let last_name = &action_hero[7..];
    // 
    // println!("{first_name}");
    // 
    // println!("{last_name}");
    // 
    // 
    // let food = "pizza";
    // println!("{}", food.len());
    // 
    // let pizza_slice = &food[0..3];
    // println!("{}", pizza_slice.len());

}
