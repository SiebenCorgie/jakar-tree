///Shows how to merge two trees together
extern crate jakar_tree;
use jakar_tree::*;
use jakar_tree::node::Attribute;


fn main() {

    //the root
    let one_root = game_tree::DefaultContent::Light("RootNode".to_string());

    // adding a tree
    let mut one_tree = tree::Tree::new(one_root, game_tree::SceneAttribute::default());

    //now adding some stuff to "RootNode"
    let one_things = vec!["One_One", "One_Two", "One_Three"];
    for thing in one_things.iter(){
        let _ = one_tree.add(game_tree::DefaultContent::Mesh(thing.to_string()), "_root".to_string(), Some(game_tree::SceneAttribute::default()));
    }

    //Also add some children to them by some more names
    let one_sub_things = vec!["One_Sub_One", "One_Sub_Two", "One_Sub_Three"];
    for thing in one_things.iter(){
        //get the node of thing and add a sub_thing to it
        for sub in one_sub_things.iter(){
            let new_name = thing.to_string() + "_" + sub;
            let _ = one_tree.add(game_tree::DefaultContent::Mesh(new_name), thing.to_string(), Some(game_tree::SceneAttribute::default()));
        }
    }

    //add a second tree
    //the root
    let two_root = game_tree::DefaultContent::Light("RootNode".to_string());

    // adding a tree
    let mut two_tree = tree::Tree::new(two_root, game_tree::SceneAttribute::default());

    //now adding some stuff to "RootNode"
    let two_things = vec!["two_One", "two_Two", "two_Three"];
    for thing in one_things.iter(){
        let _ = two_tree.add(game_tree::DefaultContent::Mesh(thing.to_string()), "_root".to_string(), Some(game_tree::SceneAttribute::default()));
    }

    //Also add some children to them by some more names
    let two_sub_things = vec!["two_Sub_One", "two_Sub_Two", "two_Sub_Three"];
    for thing in one_things.iter(){
        //get the node of thing and add a sub_thing to it
        for sub in one_sub_things.iter(){
            let new_name = thing.to_string() + "_" + sub;
            let _ = two_tree.add(game_tree::DefaultContent::Mesh(new_name), thing.to_string(), Some(game_tree::SceneAttribute::default()));
        }
    }

    println!("ATM: ", );
    one_tree.print_tree();
    println!("AND: ", );
    two_tree.print_tree();

    println!("JOINING!", );
    match one_tree.join(&two_tree, "One_One_One_Sub_One".to_string()){
        Ok(_) => {},
        Err(r) => println!("{:?}", r.to_string()),
    }
    println!("TADAA:", );
    one_tree.print_tree();
    println!("==========", );
    one_tree.print_registry();
}
