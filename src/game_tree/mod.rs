use node;

///A sample implementation of NodeContent
pub enum DefaultContent {
    Mesh(String),
    Light(String),
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


///Some example jobs
///Things a node can do
#[derive(Clone)]
pub enum Jobs {
    Translate([f32;3]),
    Rotate([f32;3]),
    Scale(f32),
}

///Some example attribte a node can have
pub struct SceneAttribute {
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: f32,
}

impl node::Attribute<Jobs> for SceneAttribute{

    fn default() -> Self{
        SceneAttribute{
            position: [0.0; 3],
            rotation: [0.0; 3],
            scale: 1.0
        }
    }

    fn execute(&mut self, job: &Jobs){
        match job{
            &Jobs::Translate(t) =>{
                self.position[0] += t[0];
                self.position[1] += t[1];
                self.position[2] += t[2];
            } ,
            &Jobs::Rotate(r) => {
                self.rotation[0] += r[0];
                self.rotation[1] += r[1];
                self.rotation[2] += r[2];
            }
            &Jobs::Scale(s) => self.scale += s,
        }
    }

    fn print_atr(&self, lvl: i32){
        for _ in 0..lvl{
            print!("\t");
        }
        println!("Attributes:", );
        for _ in 0..lvl{
            print!("\t");
        }
        println!("\tposition: {:?}", self.position);
        for _ in 0..lvl{
            print!("\t");
        }
        println!("\trotation: {:?}", self.rotation);

        for _ in 0..lvl{
            print!("\t");
        }
        println!("\tscale: {}", self.scale);
    }
}
