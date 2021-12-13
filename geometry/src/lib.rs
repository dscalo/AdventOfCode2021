#[derive(Debug, Clone)]
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

 impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}

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


#[cfg(test)]
mod tests {
    use super::*; 
    
    #[test]
    fn points_are_equal_if_x_and_y_match() {
        let point_a = Point::new(3, 4);
        let point_b = Point::new(3, 4);

        assert_eq!(point_a, point_b);
    }

    #[test]
    fn points_are_not_equal_if_x_and_y_do_not_match() {
        let point_a = Point::new(3, 4);
        let point_b = Point::new(4, 3);

        assert_ne!(point_a, point_b);
    }
}
