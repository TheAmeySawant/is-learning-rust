pub trait Draw {
    fn draw (&self);
}

pub struct Screen {
    pub components : Vec<Box<dyn Draw>>
}

impl Screen{
    pub fn run(&self){
        println!("\n\nScreen: ");
        for component in &self.components{
            component.draw();
        }
    }
}


pub struct MyScreen<T> 
where T:Draw {
    pub comps: Vec<Box<T>>
}


impl<T> MyScreen<T> 
where T:Draw {

     pub fn start(&self){
        println!("\n\nMyScreen: ");
        for comp in &self.comps {
            comp.draw();
        }
    }
}

pub struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String
}

impl Draw for Button {
    fn draw (&self) {
        //actually draw operation implementation
        println!("Button Draw method called: w: {} h: {}", self.width, self.height);
    }
}