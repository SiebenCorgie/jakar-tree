use node;
use tree;

///A public type which makes it easier to specifie a tree type
pub type TreeType = tree::Tree<DefaultContent, Jobs, SceneAttribute>;


///A sample implementation of NodeContent
#[derive(Clone)]
pub enum DefaultContent {
    Mesh(String),
    Light(String),
}

///A sample struct to show how a comapre sequenz can be cosntructed at the `attributes` implementation
/// for the `compare()` funtion.
#[derive(Clone)]
pub struct DefaultComparer {
    //if position should be compared this can be Some() else it will be ignored
    position: Option<[f32; 3]>,
    //if rotation should be compared this can be Some() else it will be ignored
    rotation: Option<[f32; 3]>,
    //if scale should be compared this can be Some() else it will be ignored
    scale: Option<f32>,
}

///Some `DefaultContent` specific funtions
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

///A sample controller for meshes
pub struct MeshController {
}



///A node controller implementation based on the Mesh Controller
impl<T,J,A> node::NodeController<T,J,A> for MeshController
where
T: node::NodeContent + Clone,
J: Clone,
A: node::Attribute<J> + Clone
{
    fn update(&mut self, node: &mut node::Node<T,J,A>){
        println!("Updating a mesh with name {}!", node.get_name());
    }
}

///And one more controller
pub struct LightController {
}
///And its implementation...
impl<T,J,A> node::NodeController<T,J,A> for LightController
where
T: node::NodeContent + Clone,
J: Clone,
A: node::Attribute<J> + Clone
{
    fn update(&mut self, node: &mut node::Node<T,J,A>){
        println!("Updating a light with name {}!", node.get_name());
    }
}




///The implementation of the NodeContent for the DefaultContent
impl node::NodeContent for DefaultContent{
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
    fn update<SceneAttribute>(&mut self, attributes: &SceneAttribute){
        //println!("UpusDaterus", );
    }
}


///Some example jobs
///Things a node can do
#[derive(Clone)]
pub enum Jobs {
    Translate([f32;3]),
    Rotate([f32;3]),
    Scale(f32),
}

///Some example attribte a node can have
#[derive(Clone)]
pub struct SceneAttribute {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: f32,
}
///The implementation which qualifies this as a ndoe attribute. Be sure to implement the comparing
/// correctly
impl node::Attribute<Jobs> for SceneAttribute{
    type Comparer = DefaultComparer;

    fn default() -> Self{
        SceneAttribute{
            position: [0.0; 3],
            rotation: [0.0; 3],
            scale: 1.0
        }
    }

    fn execute(&mut self, job: &Jobs) -> Jobs{
        match job{
            &Jobs::Translate(t) =>{
                self.position[0] += t[0];
                self.position[1] += t[1];
                self.position[2] += t[2];
                Jobs::Translate(t)
            } ,
            &Jobs::Rotate(r) => {
                self.rotation[0] += r[0];
                self.rotation[1] += r[1];
                self.rotation[2] += r[2];
                Jobs::Rotate(r)
            }
            &Jobs::Scale(s) => {
                self.scale += s;
                Jobs::Scale(s)
            },
        }
    }

    fn print_atr(&self, lvl: i32){
        for _ in 0..lvl + 1{
            print!("\t");
        }
        println!("Attributes:", );
        for _ in 0..lvl + 1{
            print!("\t");
        }
        println!("\tposition: {:?}", self.position);
        for _ in 0..lvl + 1{
            print!("\t");
        }
        println!("\trotation: {:?}", self.rotation);

        for _ in 0..lvl + 1{
            print!("\t");
        }
        println!("\tscale: {}", self.scale);
    }

    fn compare(&self, comp: &Self::Comparer) -> bool{
        let mut status = true;
        //position
        match comp.position{
            Some(pos) => {
                if pos != self.position{
                    status = false;
                }
            },
            None => {}
        }
        //rotation
        match comp.rotation{
            Some(rot) => {
                if rot != self.rotation{
                    status = false;
                }
            },
            None => {}
        }
        //scale
        match comp.scale{
            Some(sca) => {
                if sca != self.scale{
                    status = false;
                }
            },
            None => {}
        }


        //scale
        status

    }
}
