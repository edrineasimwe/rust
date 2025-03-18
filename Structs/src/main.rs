fn main() {
    let rectangle1 = Rectangle{
        width: 9.0,
        length: 10.0
    };
    let react_area = area(rectangle1.width, rectangle1.length);
    let react_parameter   = parameter(rectangle1.width, rectangle1.length);


    println!("the area of a rectangle is {:#?}", react_area);
    println!("the parameter of a rectangle is {:#?}", react_parameter);

    println!("the area of a rectangle is {:#?}", rectangle1.area());
    println!("the parameter of a rectangle is {:#?}", rectangle1.parameter());


}

#[derive(Debug)]
struct Rectangle{
    width:f64,
    length:f64,
}

fn area(width:f64, length:f64) -> f64{
    width*length
}

fn parameter(length:f64, width:f64) -> f64{
    2.0 * (width + length)
}


impl Rectangle {
    fn area(&self) -> f64{
        self.width*self.length

    }
    fn parameter(&self) -> f64{
        2.0 * (self.width + self.length)

    }
}