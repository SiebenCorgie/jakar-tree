use std::collections::BTreeMap;
use tree;
use std::sync::{Arc, Mutex};



///Attributes of an object can be anything. But they must be able to perform the Jobs `J` and to compare them self to C
pub trait Attribute<J: Clone> {
    ///The type used to comapre attributes with each other
    type Comparer;
    ///Creates a default attribute set
    fn default() -> Self;
    ///Execute the `job` on `self`. And returns the job which should be added to each child.
    ///Why?
    /// Well consider the following  case:
    /// You want to rotate a object, but each child should be rotated in relation to this object.
    /// With this way you could call `rotate()` on this object, but pass a `rotate_around_point()`
    /// down to the children.
    fn execute(&mut self, job: &J) -> J;
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
    ///Gets called on an update of this node. gets the `jobs` of this node. Be sure to spawn threads for heavyer work load.
    fn update<J>(&mut self, jobs: &mut Vec<J>);
}


///Describes a node for a `Tree`. Each Node can have child nodes as well as ONE value.
#[derive(Clone)]
pub struct Node<T: NodeContent + Clone, J: Clone, A: Attribute<J> + Clone> {
    ///The name of this node
    name: String,
    ///The value of this node
    value: T,
    ///Contains all children of this node sorted by name.
    children: BTreeMap<String, Node<T, J, A>>,
    ///Contains a list of things this node has to do when updated next
    jobs: Vec<J>,
    ///Can contain any type of attributes. Any `Job` can be applied to an attributes field.
    attributes: A,
}


impl<T: NodeContent + Clone, J:  Clone, A: Attribute<J> + Clone> Node<T, J, A>{
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

    ///Copys the name, value, attributes and tick closure into a new node object. Children and jobs are
    /// reseted.
    pub fn copy(&self) -> Self{
        Node{
            name: self.name.clone(),
            value: self.value.clone(),
            children: BTreeMap::new(),
            jobs: Vec::new(),
            attributes: self.attributes.clone(),
        }
    }

    ///Returns the an `Ok(&mut Node)` at `path` if there is one at this location, or `Err()` if not.
    pub fn get_node(&mut self, path: &mut Vec<String>) -> Result<&mut Self, tree::NodeErrors> {
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


    /// Applys `parent_jobs` first, then applies the jobs of `self.jobs`,
    /// finally sends both to all children. `delta` is the time in seconds since the last update.
    ///It will also execute the update function of this nodes value.
    pub fn update(&mut self, delta: f32, parent_jobs: &Vec<J>){

        self.value.update(&mut self.jobs);


        //first construct the final job vector to apply.
        //we clone the job because we don't want to apply jobs of one children to all children.
        // the append(self.jobs) will also empty self.jobs. This leaves room for adding new ones.
        let mut job_vec = parent_jobs.clone();
        job_vec.append(&mut self.jobs);

        //now apply it ordered
        for job in job_vec.iter_mut(){
            *job = self.attributes.execute(&job);
        }

        //now send them to the children
        for (_, child) in self.children.iter_mut(){
            child.update(delta, &job_vec);
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

    ///Takes self's values and constructs a new node in the new `parent_tree` tree at the `parent_node`.
    /// Then adds its children to the new returned node.

    /// # Note
    /// if there is an error while adding self, no further children will be added
    pub fn join(&self, parent_tree: &mut tree::Tree<T, J, A>, parent_node: String)
    -> Result<(), tree::NodeErrors>{
        //first of all we need to add self to the new parent tree. This is necessary to keep the
        // current hierachy.
        let new_name =
        match parent_tree.add(
            self.value.clone(),
            parent_node,
            Some(self.attributes.clone())
        ){
            Ok(new_n) => new_n,
            Err(er) => return Err(er),
        };



        for (_, child) in self.children.iter(){
            //now pass the new name together with the tree down
            let adding_status = child.join(parent_tree, new_name.clone());
            //return the error if something went wrong
            match adding_status{
                Ok(_) => {},
                Err(r) => return Err(r),
            }
        }

        Ok(())
    }

    ///Returns a copy of the name.
    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    ///Returns a reference to the value of this node
    pub fn get_value(&self) -> &T{
        &self.value
    }

    ///Returns a mutable reference to this nodes value
    pub fn get_value_mut(&mut self) -> &mut T{
        &mut self.value
    }

    ///Returns the current job vector of this node
    pub fn get_jobs(&self) -> &Vec<J>{
        &self.jobs
    }

    ///Returns a reference to the current atrributes.
    pub fn get_attrib(&self) -> &A{
        &self.attributes
    }

    ///Returns a mutable reference to the current atrributes. Keep in mind that
    /// you usually should change them through jobs since thoose are pushed
    /// down to the children as well.
    pub fn get_attrib_mut(&mut self) -> &mut A{
        &mut self.attributes
    }

    ///Returns a reference to the children
    pub fn get_children(&self) -> &BTreeMap<String, Node<T,J,A>>{
        &self.children
    }

    ///Returns the children as well, but mutable. Be careful what you do!
    pub fn get_children_mut(&mut self) -> &mut BTreeMap<String, Node<T,J,A>>{
        &mut self.children
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
