// Associate functions and methods
// Associated function are functions that are defined on a type generally, while methods are associated functions that are called on a particular instance of a type;
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    //  This is an associated function because this function is associated with a particular type, that is, Point;
    // Associated function don't need to be called with an instance.
    // These functions are generally used like constructors.
    pub fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    //   another assocaited function
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }
}

pub struct Rectangle {
    pub p1: Point,
    pub p2: Point,
}

impl Rectangle {
    // This is a method
    // '&self' is sugar for "self:&Self", where "Self" is the typeof the caller object. In this case 'Self'='Rectangle'

    pub fn area(&self) -> f32 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        ((x1 - x2) * (y1 - y2)).abs()
    }

    pub fn perimeter(&self) -> f32 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
    // Function item:- Function items are function names along with their parameter types and return types. They can be assigned o variables and called directly.
    // Function pointer:- Function pointer are variables that hold memory address of functions. They allow for indirect function calls.
    // Function traits:- 

    // closures:-  Closures are functions that can capture the enclosing environment.
    
}
