struct Rect{
    width:u64,
    height:u64
}

impl Rect{
    fn has_width(&self) -> bool{
        self.width > 0
    }
    fn has_height(&self) -> bool{
        self.height > 0
    }
    fn area(&self) -> u64{
        if self.has_height() && self.has_width(){
            self.width * self.height
        }else{
            0
        }
    }
    fn can_contain(&self, other:&Rect) -> bool{
        self.width > other.width && self.height > other.height
    }
    fn set_square(size:u64) -> Self{
        Self{
            width: size,
            height: size
        }
    }
}

fn main() {
    let rectangle1 = Rect::set_square(20);
    let rectangle2 = Rect{
        width:20,
        height:10
    };
    println!("Area of rect 1 is: {}", rectangle1.area());
    println!("Area of rect 2 is: {}", rectangle2.area());
    if rectangle1.can_contain(&rectangle2){
        println!("Rect 1 can contain rect 2");
    }else if rectangle2.can_contain(&rectangle1){
        println!("Rect 2 can contain rect 1");
    }else{
        println!("Rects cannot fit inside each other.");
    }
}
