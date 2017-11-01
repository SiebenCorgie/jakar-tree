use std::time::{Instant, Duration};
extern crate scene_tree;
use scene_tree::*;
use scene_tree::node::Attribute;


fn main() {

    //the root
    let root = game_tree::DefaultContent::Light("RootNode".to_string());

    // adding a tree
    let mut tree = tree::Tree::new(root, game_tree::SceneAttribute::default());

    //now adding some stuff to "RootNode"
    let things = vec!["Mesh", "Teddy", "other_stuff", "Test"];
    for thing in things.iter(){
        tree.add(game_tree::DefaultContent::Mesh(thing.to_string()), "_root".to_string(), Some(game_tree::SceneAttribute::default()));
    }

    //Also add some children to them by some more names
    let sub_things = vec!["Subby", "TheThird", "Saeft", "MyPeopleNeedMe", "Oi"];
    for thing in things.iter(){
        //get the node of thing and add a sub_thing to it
        for sub in sub_things.iter(){
            let new_name = thing.to_string() + "_" + sub;
            tree.add(game_tree::DefaultContent::Mesh(new_name), thing.to_string(), Some(game_tree::SceneAttribute::default()));
        }
    }

    //npow print
    tree.print_tree();

    //print the paths
    tree.print_registry();

    //getting a thing in time
    let start_time = Instant::now();
    let thiungy = tree.get_node("Test_Oi".to_string());
    let time_needed = start_time.elapsed().subsec_nanos() as f32 / 1_000_000_000.0;
    println!("Time 01: {}", time_needed);

}
