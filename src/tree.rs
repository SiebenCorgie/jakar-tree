use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Instant;
use std;

use node;
///The errors which can appear when adding a new child
#[derive(Debug)]
pub enum NodeErrors {
    ///Appears if there is no child with the given search parameter
    NoSuchChild(String),
    NoNodeFound(String),
}

///Implements a compfy to_string methode
impl std::string::ToString for NodeErrors{
    fn to_string(&self) -> String{
        match self{
            &NodeErrors::NoSuchChild(ref s) => s.clone(),
            &NodeErrors::NoNodeFound(ref s) => s.clone(),
        }
    }
}

///Describes a tree which can hold nodes of the type T.
/// The tree also holds a registry of all its values with its paths.
#[derive(Clone)]
pub struct Tree<T: node::NodeContent + Clone, J: Clone, A: node::Attribute<J> + Clone>{

    ///The name of this tree. It is usally derived from the name of the root node supplyied to
    //the `new()` function. But you can chagne the name without breaking anythign if you want.
    pub name: String,

    ///Stores the path to every node of this tree, keyed by the nodes name.
    /// For instance a data set could look like this:
    ///
    /// "Teddy", "Root/Cave/LeftSide/Teddy"
    ///
    /// In this case the teddy is At the root nodes child "Cave", which has a child LeftSide, which has the child Teddy
    pub registry: BTreeMap<String, PathBuf>,
    ///The root node of this tree
    pub root_node: node::Node<T, J, A>,
    ///Keeps track of the last tick time.
    last_tick: Instant,

}

///Implements the base functions of `Tree`
impl<T: node::NodeContent + Clone, J: Clone, A: node::Attribute<J> + Clone> Tree<T, J, A> {

    ///Creates a new tree with an `root` node with set `attributes`
    pub fn new(root: T, root_attributes: A) -> Self{

        let tree_name = root.get_name();
        let root_node = node::Node::new(root, root_attributes);
        let mut registry = BTreeMap::new();
        //add the root node to the registry
        registry.insert(tree_name.clone(), PathBuf::from("/".to_string()));

        Tree{
            name: tree_name,
            registry: registry,
            root_node: root_node,
            last_tick: Instant::now(),
        }
    }

    ///Same as `add()` but the `new_child` will atomaticly be added to the root node
    pub fn add_at_root(&mut self, new_child: T, attributes: Option<A>)->
    Result<String, NodeErrors>{
        //get the root name from the tree name (see implementation of the new() function)
        let root_name = self.name.clone();
        //now add it there
        self.add(new_child, root_name, attributes)
    }

    ///Adds a `new_child` at a `parent` node with `Some(attributes)` set
    /// (or the default attributes if None is supplied).
    /// Returns the name under which it was addded as `Ok(name)`
    /// or an `Err(e)` if something went wrong.
    pub fn add(&mut self, new_child: T, parent_name: String, attributes: Option<A>)->
    Result<String, NodeErrors>{
        //First we have to get the node in this tree with the searched name.
        // If this is successful, we test the new_child's name for being unique.
        // If not, we change the name to something unique for the `T`.
        // After this we add the unique named node to the parent.
        // The we add the new, unique name as well as the new path to the registry.
        //Finally we return the unique name in an Ok(k).

        let parent_path = {
            match self.registry.get(&parent_name){
                Some(parent_path) =>{
                    //we have to copy this path because we are going to mess with it quiet much but
                    // don't want to change it within the registry
                    parent_path.clone()
                }
                None =>{
                    //okay we already catched an error while getting the parent. Returning the error
                    return Err(NodeErrors::NoSuchChild(
                        //constructing the error message
                        String::from("Could not find ") + &parent_name + "in tree!"
                    ));
                }
            }
        };

        //Testing the childs name
        let unique_name: String = {
            //Checking for the name. NOTE: to make the system correctly working, we have to
            // replace dots "." with something. Otherwise the name would make the node's path to a file
            // which would screw around with the registry
            match self.registry.get(&new_child.get_name().replace(".", "_")){
                Some(_) => {
                    //The name is already in there, we have to make a new unique one
                    //Currently we use the easiest way to make it unique by adding an incrementing
                    //number to the end of this name until we can't find a entry with this name+number
                    //then using this name+number as unique name.
                    let mut append_number = 0;
                    let initial_node_name = new_child.get_name().replace(".", "_");
                    while self.registry.get(
                                &(initial_node_name.clone() + "_" + &append_number.to_string())
                                            ).is_some()
                    {
                        append_number +=1;
                    }
                    //after finding a good enough number, returning the name
                    initial_node_name.clone() + "_" + &append_number.to_string()
                },
                None => {
                    //the name is already unique returing it
                    new_child.get_name().replace(".", "_").clone()
                }
            }
        };

        //Pre constructing the new node path.
        let mut new_path = parent_path.clone();
        new_path.push(unique_name.clone());

        //something could be wrong with the path or so (shouldnt but still I don't like unwraps())
        match self.get_from_path(&parent_path){
            Ok(k) => {
                //we got the right parent, going to add the child to it
                //but before adding we have to decide if we can take the supplied attributes or
                // if we have to create a set of default ones.
                let atrib = {
                    match attributes{
                        Some(atr) => atr,
                        None => A::default(),
                    }
                };

                k.add_with_name(new_child, unique_name.clone(), atrib);
            },
            //Otherwise return with an error
            Err(e) => {
                println!("Error while adding a new node at {}: {}", parent_name, e.to_string());
                return Err(NodeErrors::NoSuchChild("Could not find such a child".to_string()));
            }
        }

        //if we got to this point we can be sure that we have a unique name and
        // the right parent node.So we can add the path to the registry.
        self.registry.insert(unique_name.clone(), new_path);

        Ok(unique_name)
    }

    ///Updates the whole tree
    pub fn update(&mut self){
        //Get the new time since the last tick
        let time = self.last_tick.elapsed().subsec_nanos() as f32 / 1_000_000_000.0;
        self.last_tick = Instant::now();

        self.root_node.update(time, &Vec::new());
    }

    ///Returns a mutable reference to a child by its `path`
    fn get_from_path(&mut self, path: &PathBuf) -> Result<&mut node::Node<T, J, A>, NodeErrors>{
        //To get a node we walk down the path by searching (.pop()) for the last element of the vector
        //which we get by transforming the path into custom_path_iter().
        //the node get the changes path (one item smaller) and searches in its own nodes for a node
        //of the "this time" last elment etc. till we got the right node. This one is returned till
        //here and then returned to the caller.
        let mut reverse_path = custom_path_iter(path);

        //Test if the first is the root node, if yes, add to it, otherwise go down the root node
        if reverse_path.len() == 0{
            return Ok(&mut self.root_node);
        }

        self.root_node.get_node(&mut reverse_path)
    }

    ///Returns a node with this `name`
    pub fn get_node(&mut self, name: &str) -> Option<&mut node::Node<T, J, A>>{
        //get the nodes path, if there is such a node, return it as Some(T) else return None
        let path = {
            match self.registry.get(name){
                Some(path) => path.clone(),
                None => return None,
            }
        };

        match self.get_from_path(&path){
            Ok(n) => Some(n),
            Err(e) => {
                println!("There was a problem while getting a node: {}", e.to_string());
                None
            },
        }
    }

    ///Returns true if this Tree contains a node with this name
    pub fn has_node(&self, node_name: &str) -> bool{
        self.registry.contains_key(&String::from(node_name))
    }

    ///Same as `join()`, but it automaticly integrates `tree` at the root node
    pub fn join_at_root(&mut self, tree: &Self) -> Result<(),NodeErrors>{
        //the root name
        let root_name = self.name.clone();
        println!("Adding at: {}", root_name);
        self.join(tree, &root_name)
    }


    ///Merges `self` into `tree` at the node with a `name`. Returns Ok(k) if
    /// everything went all right or Err(e) if something went wrong.
    /// NOTE: All values and attributes are cloned.
    pub fn join(&mut self, tree: &Self, name: &str) -> Result<(),NodeErrors>{

        //Try to get the root node, add it at "name", get the actual returning name, add the children there etc
        let new_root_name = {
            match self.add(
               tree.root_node.get_value().clone(),
               name.to_string(),
               Some(tree.root_node.get_attrib().clone()),
           ){
               Ok(new_name) => new_name,
               Err(r) => return Err(r),
           }
       };


       let mut return_val = Ok(());

        //now for all root children, add them to the "new_root_name" and redo the procedure
        for (_, child) in tree.root_node.get_children().iter(){
            match child.join(self, new_root_name.clone()){
                Ok(_) => {},
                Err(e) => return_val = Err(e),
            }
        }

        return_val
        //tree.root_node.join(self, new_root_name)
    }

    ///Prints a debug tree of the things in this tree
    pub fn print_tree(&self){
        //init the base level then go though all children of the root node
        let level = 0;
        let mut node_count = 0;
        self.root_node.print_debug(level, &mut node_count);

        println!("There are {} nodes in the tree!", node_count);

    }

    //prints the current registry
    pub fn print_registry(&self){
        println!("Current Registry:", );
        for (k, i) in self.registry.iter(){
            println!("\t {} -> \n \t\t{:?}", k, i);
        }
        println!("=====\n Tree has {} entries!\n =====", self.registry.len());
    }

}

///Generates a vector which holds the root of an path as the last element and the last node as the
/// the first element.
///For instance: \n
/// `"/home/teddy/bear"` becomes an vector of `vec!["bear", "teddy", "home"]`.
fn custom_path_iter(path: &PathBuf) -> Vec<String>{

    let mut mut_parents = path.clone();


    let mut is_some = true;
    let mut wrong_way_vec = Vec::new();

    while is_some {
        let parent = {
            match mut_parents.file_stem(){
                Some(e) =>{
                    e.to_str().unwrap().to_string()
                },
                None => {
                    break;
                }
            }
        };
        //add the valid parent to the vector
        wrong_way_vec.push(parent);
        //now pop out the parent for the next one
        is_some = mut_parents.pop();
    }
    wrong_way_vec
}
