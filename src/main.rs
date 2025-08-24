use std::os::unix::fs::lchown;

fn do_hero_stuff(hero_name: &str) {
    println!("{hero_name} saves the day!");
}

fn main() {

    // let action_hero = String::from("Sathya Ram");
    //
    // do_hero_stuff(&action_hero);
    //
    // let another_hero = "Vinothini";
    // do_hero_stuff(&another_hero);
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

//     Array Slices
//     let values = [4, 8, 15,16,23, 42];
//
//     let my_slice = &values[1..];
//     println!("{my_slice:?}");

//     Deref Coercision with array slices
//     let values = [4, 8, 15,16,23, 42];
//     let regular_conference = &values;
//     print_length(regular_conference);
//     let slice_of_three = &values[..3];
//     print_length(slice_of_three);

//     Mutable arrays slices
    let mut my_array = [10, 15,20, 25, 30];
    let my_slice = &mut my_array[2..4];
    
    println!("{my_slice:?}");
    
    my_slice[0] = 100;
    
    println!("{my_slice:?}");

}

fn print_length(reference: &[i32]){
    println!("{}", reference.len());
}
