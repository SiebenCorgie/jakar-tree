extern crate jakar_tree;
use jakar_tree::*;
use jakar_tree::node::Attribute;
mod game_tree;

fn main(){
    //the root
    let one_root = game_tree::DefaultContent::Light("RootNode".to_string());

    // adding a tree
    let mut one_tree = tree::Tree::new(one_root, game_tree::SceneAttribute::default());

    //now adding some stuff to "RootNode"
    let one_things = vec!["One_One", "One_Two.dot", "One_Three.rold"];
    let mut last_name = String::from("ted");
    for thing in one_things.iter(){
        last_name = one_tree.add_at_root(
            game_tree::DefaultContent::Mesh(thing.to_string()),
            Some(game_tree::SceneAttribute::default())
        ).expect("failed to add new node");
    }

    one_tree.add(
        game_tree::DefaultContent::Mesh("Rolfy".to_string()),
        last_name,
        None
    );

    one_tree.print_tree();

}
