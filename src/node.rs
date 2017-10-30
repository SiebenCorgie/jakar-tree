use std::collections::BTreeMap;
use tree;

///Things a node can do
pub enum Jobs {
    Translate([f32;3]),
    Rotate([f32;3]),
    Scale([f32;3]),
}

///A sample implementation of NodeContent
pub enum DefaultContent {
    Mesh(String),
    Light(String),
}

impl NodeContent for DefaultContent{
    ///Should return the name of this content
    fn get_name(&self) -> String{
        match self {
            &DefaultContent::Mesh(ref x) =>{
                x.clone()
            }
            &DefaultContent::Light(ref x) =>{
                x.clone()
            }
        }
    }

    /// Returns the bounds of this objects
    fn get_bounds(&self) -> ([f32;3] , [f32;3]){
        match self {
            &DefaultContent::Mesh(_) =>{
                ([0.0; 3], [1.0; 3])
            }
            &DefaultContent::Light(_) =>{
                ([0.0; 3], [1.0; 3])
            }
        }
    }

}

impl DefaultContent{
    ///returns Some(Mesh) if self is a mesh or none if not
    pub fn as_mesh(&self) -> Option<&String>{
        match self {
            &DefaultContent::Mesh(ref x) =>{
                Some(x)
            }
            &DefaultContent::Light(_) =>{
                None
            }
        }
    }
}


///Each type which implements `NodeContent` can be stored in a `Node`.
///
/// **Why cant I change * of this object?**
/// Well, i decided against it because we use the names of objects to generate a node
/// name. But considering the following case, name chagne would be useless: You got a type `Arc<Mutex<Type>>`
/// which you want to store in a node. But, you could store this value twice. If you would rename the `Type` everytime
/// the first time when adding to name_0, the next node containing the `Arc<Mutex<Type>>` would be named name_0_0, and the next
/// one would be name_0_0_0 etc. That's why you cant change the types from the trait.
pub trait NodeContent {
    ///Should return the name of this content
    fn get_name(&self) -> String;
    /// Returns the bounds of this objects
    fn get_bounds(&self) -> ([f32;3] , [f32;3]);
}

///Describes a node for a `Tree`. Each Node can have child nodes as well as ONE value.
pub struct Node<T: NodeContent> {
    ///The name of this node
    pub name: String,
    ///The value of this node
    pub value: T,
    ///Contains all children of this node sorted by name.
    pub children: BTreeMap<String, Node<T>>,
    ///Contains a list of things this node has to do when updated next
    pub jobs: Vec<Jobs>,
}

impl<T: NodeContent> Node<T>{

    ///Create a new node from a `value`, returns this node.
    pub fn new(value: T) -> Self{
        Node{
            name: value.get_name(),
            value: value,
            children: BTreeMap::new(),
            jobs: Vec::new()
        }
    }


    ///Adds `new` as child of `self`.
    pub fn add_with_name(&mut self, new: T, name: String){

        //Create the node from child
        let new_child_node = Node{
            name: name.clone(),
            value: new,
            children: BTreeMap::new(),
            jobs: Vec::new()

        };
        //add the child to self
        self.children.insert(name, new_child_node);

    }

    ///Returns the an `Ok(&mut Node)` at `path` if there is one at this location, or `Err()` if not.
    pub fn get_node(&mut self, path: &mut Vec<String>) -> Result<&mut Node<T>, tree::NodeErrors> {
        //get the name of the child we are searching for if we get an `None` from the pop(),
        // it means we reached the last child, so we return the node with this name as &mut T
        // otherwise we search in the node with the name optained from pop() for the node ref.

        let child_name = match path.pop(){
            None =>{
                //well this should be the node we searched for, returning &mut self
                return Ok(self)
            },
            Some(node_with_this_name) => node_with_this_name,
        };

        //It seams like we did not reach the end of the path, so we try to optain the node with the
        //name from the .pop().
        let node_with_name = {
            match self.children.get_mut(&child_name){
                //nice we got a child with this name
                Some(child) => child,
                //there is no such child, returning an error
                None => return Err(tree::NodeErrors::NoNodeFound(String::from("could not find node in children"))),
            }
        };

        //returning whatever we got from this child with the modified path
        node_with_name.get_node(path)
    }

    ///Prints self and then all children a level down and so on, creates a nice tree print out
    pub fn print_debug(&self, lvl: i32){
        //print self then go though all children
        for _ in 0..lvl{
            //print the tabs for self
            print!("\t");
        }
        //then print the name of self
        println!("{}", self.name);

        //no go though children
        for (_, child) in self.children.iter(){
            child.print_debug(lvl + 1);
        }
    }

}
