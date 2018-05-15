///Tests the speed of this tree in extreme situations, you'll find the latest tests below
//=========
//Test from pre Arc phase, everything was on the stack
// Needed 12.461858ms to fill tree with 157 nodes!
// Needed 0.00353ms to get a non existent node!
// Needed 0.086341ms to merge tree! now has 162 nodes
//=========


mod game_tree;

extern crate jakar_tree;
use jakar_tree::*;
use jakar_tree::node::*;
use jakar_tree::node::Node;

use std::time::Instant;

type NodeType = Node<game_tree::DefaultContent, game_tree::Jobs, game_tree::SceneAttribute>;


fn main(){
    //We'll do 4 level with configurable amounts of items. We then get a single item and merge a small
    //tree.

    let levels = vec![4,2,3,5];

    //the root
    let root = game_tree::DefaultContent::Light("RootNode".to_string());

    // adding a tree
    let mut tree = tree::Tree::new(root, game_tree::SceneAttribute::default());

    let mut time = Instant::now();

    let mut name = String::from("Teddy");

    for one in 0..levels[0]{

        let one_node = game_tree::DefaultContent::Mesh(one.to_string() + "_mesh");
        let one_node_name = tree.add_at_root(one_node, None).unwrap();

        name = one_node_name.clone();

        for two in 0..levels[1]{
            let two_node = game_tree::DefaultContent::Mesh(two.to_string() + "_mesh");
            let two_node_name = tree.add(two_node, one_node_name.clone(), None).unwrap();

            for three in 0..levels[2]{
                let three_node = game_tree::DefaultContent::Mesh(three.to_string() + "_mesh");
                let three_node_name = tree.add(three_node, two_node_name.clone(), None).unwrap();

                for four in 0..levels[3]{
                    let four_node = game_tree::DefaultContent::Mesh(four.to_string() + "_mesh");
                    let four_node_name = tree.add(four_node, three_node_name.clone(), None).unwrap();
                }
            }
        }
    }

    println!("Needed {}ms to fill tree with {} nodes!", time.elapsed().subsec_nanos() as f32 / 1_000_000.0, tree.registry.len());
    time = Instant::now();
    //tree.print_tree();
    let _ = tree.get_node("Teddy");
    println!("Needed {}ms to get a non existent node!", time.elapsed().subsec_nanos() as f32 / 1_000_000.0);

    tree.update();
    tree.get_node(&name).unwrap().get_attrib_mut().scale = 10.0;
    tree.update();

    let mut new_tree = tree::Tree::new(
        game_tree::DefaultContent::Light("RootNodeDuos".to_string()),
        game_tree::SceneAttribute::default()
    );

    new_tree.add_at_root(
        game_tree::DefaultContent::Light("MeshyMeshMesh".to_string()),
        None
    );
    new_tree.add_at_root(
        game_tree::DefaultContent::Mesh("MeshyTheSecond".to_string()),
        None
    );
    new_tree.add_at_root(
        game_tree::DefaultContent::Light("Tedberg".to_string()),
        None
    );
    new_tree.add_at_root(
        game_tree::DefaultContent::Light("Rudolf".to_string()),
        None
    );

    time = Instant::now();

    let _ = tree.join(&new_tree, "RootNode");

    println!("Needed {}ms to merge tree! now has {} nodes", time.elapsed().subsec_nanos() as f32 / 1_000_000.0, tree.registry.len());
    time = Instant::now();






}
