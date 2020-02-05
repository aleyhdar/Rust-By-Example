struct Rectangle {
    width: u8,
    height: u8
} 
impl Rectangle {
    fn print_description(&self) {
        println!("Rectangle: {} x {}", self.width, self.height);
    }
    fn rect_field(&self) -> u8 {
        return (self.width)*(self.height)
    }
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}
fn main() {
    let my_rect = Rectangle {width: 10, height: 20};
    my_rect.print_description();
    println!("Rectangle is a square: {} and field: {}", my_rect.is_square(), my_rect.rect_field());

}
