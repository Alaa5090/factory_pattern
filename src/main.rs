trait Shape {
    fn drow(&self);
}
struct Rectangle{}
impl Shape for Rectangle {
fn drow(&self) {
    println!("this is rectangle");
}
    }
struct Circle{}
impl Shape for Circle {
fn drow(&self) {
    println!(" this is circle");
}    
  
}
enum ShapeType {
    Rectangle,
    Circle
}
struct ShapeFactory{}
impl ShapeFactory {
    fn new_shape(s:&ShapeType)->Box<dyn Shape>
     {
         match s {
             ShapeType::Circle=>CircleFactory::new(),
             ShapeType::Rectangle=>RectangleFactory::new()
         }
        
    }
}
    
struct RectangleFactory{}
impl RectangleFactory {
    fn new () ->Box<dyn Shape> {
        Box::new(Rectangle{})
    }
}

struct CircleFactory{}
impl CircleFactory {
    fn new () ->Box<dyn Shape> {
        Box::new(Circle{})
    }
}



fn main() {
    println!("Hello, world!");
    let my_shape=ShapeFactory::new_shape(&ShapeType::Circle);
    my_shape.drow();

    let my_shape=ShapeFactory::new_shape(&ShapeType::Rectangle);
    my_shape.drow();

}
