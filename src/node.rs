use std::collections::BTreeMap;
use tree;


///Attributes of an object can be anything. But they must be able to perform the Jobs `J` and to compare them self to C
pub trait Attribute<J: Clone> {
    ///The type used to comapre attributes with each other
    type Comparer;
    ///Creates a default attribute set
    fn default() -> Self;
    ///Execute the `job` on `self`.
    fn execute(&mut self, job: &J);
    ///Should print the content of self in an readable form.
    fn print_atr(&self, lvl: i32);
    ///Returns true if `self` matches the supplied `attributes`
    fn compare(&self, attributes: &Self::Comparer) -> bool;
}

///Each type which implements `NodeContent` can be stored in a `Node`.
///
/// **Why can't I change * of this object?**
/// Well, i decided against it because we use the names of objects to generate a node
/// name. But considering the following case, name change would be useless: You got a type `Arc<Mutex<Type>>`
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
pub struct Node<T: NodeContent, J: Clone, A: Attribute<J>> {
    ///The name of this node
    pub name: String,
    ///The value of this node
    pub value: T,
    ///Contains all children of this node sorted by name.
    pub children: BTreeMap<String, Node<T, J, A>>,
    ///Contains a list of things this node has to do when updated next
    pub jobs: Vec<J>,
    ///Can contain any type of attributes. Any `Job` can be applied to an attributes field.
    pub attributes: A,
}

impl<T: NodeContent, J:  Clone, A: Attribute<J>> Node<T, J, A>{
    ///Create a new node from a `value` and an `attribute`, returns this node.
    ///The name is retriefed from the nodes `get_name()` function.
    /// #unsave
    /// This is unsave when used standalone to add this node to a tree.
    pub fn new(value: T, attribute: A) -> Self{
        Node{
            name: value.get_name(),
            value: value,
            children: BTreeMap::new(),
            jobs: Vec::new(),
            attributes: attribute,
        }
    }


    ///Adds `new` as child of `self` with `attribte`.
    pub fn add_with_name(&mut self, new: T, name: String, attribute: A){

        //Create the node from child
        let new_child_node = Node{
            name: name.clone(),
            value: new,
            children: BTreeMap::new(),
            jobs: Vec::new(),
            attributes: attribute,
        };
        //add the child to self
        self.children.insert(name, new_child_node);

    }

    ///Returns the an `Ok(&mut Node)` at `path` if there is one at this location, or `Err()` if not.
    pub fn get_node(&mut self, path: &mut Vec<String>) -> Result<&mut Node<T,J,A>, tree::NodeErrors> {
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

    ///Applys `parent_jobs` first, then applies the jobs of `self.jobs`,
    /// finally sends both to all children.
    pub fn update(&mut self, parent_jobs: &Vec<J>){
        //first construct the final job vector to apply.
        //we clone the job because we don't want to apply jobs of one children to all children.
        // the append(self.jobs) will also empty self.jobs. This leaves room for adding new ones.
        let mut job_vec = parent_jobs.clone();
        job_vec.append(&mut self.jobs);

        //now apply it ordered
        for job in job_vec.iter(){
            self.attributes.execute(job);
        }

        //now send them to the children
        for (_, child) in self.children.iter_mut(){
            child.update(&job_vec);
        }
    }

    ///Adds a job to this node
    pub fn add_job(&mut self, job: J){
        self.jobs.push(job);
    }

    ///Immidiatly executes this job onto the node, usually used when setting up a node with default settings.
    pub fn job_exec(&mut self, job: J){
        self.attributes.execute(&job);
    }


    ///Prints self and then all children a level down and so on, creates a nice tree print out
    pub fn print_debug(&self, lvl: i32, counter: &mut u32){
        //still need the dereferencing :/
        *counter = *counter + 1;

        //print self then go though all children
        for _ in 0..lvl{
            //print the tabs for self
            print!("\t");
        }
        //then print the name of self
        println!("{}", self.name);
        //then add the offset again, but print the attributes this time

        self.attributes.print_atr(lvl);

        //no go though children
        for (_, child) in self.children.iter(){
            child.print_debug(lvl + 1, counter);
        }
    }

}
