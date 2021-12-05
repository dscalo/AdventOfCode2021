pub struct Point {
    pub x: i32,
    pub y: i32,
 }
 
 impl Point {
     pub fn new(x: i32, y: i32) -> Point {
         Point { x: x, y: y }
     }

     pub fn pretty_print(&self)  {
         println!("x: {}, y: {}",self.x, self.y)
     }

     pub fn to_string(&self) -> String {
         format!("x: {}, y: {}", self.x, self.y)
     }
    
 }

pub struct Line {
    pub start: Point,
    pub end: Point
}

impl Line {
   pub fn new (s: Point, e: Point) -> Line {
        Line{start: s, end: e}
    }

    pub fn pretty_print(&self) {
        println!("Start: {}, End: {}", self.start.to_string(), self.end.to_string())
    }
}
