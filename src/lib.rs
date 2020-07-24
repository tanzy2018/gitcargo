#[allow(unused)]
pub fn five()->i32{
    5
}
#[allow(unused)]
pub struct Point{
    x:i32,
    y:i32,
}
#[allow(unused)]
impl Point {
    pub fn new(x:i32,y:i32)->Self{
         Point{
             x,
             y,
         } 
    }
}