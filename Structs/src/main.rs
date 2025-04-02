// fn main() {
    
//     // let react_area = area(rectangle1.width, rectangle1.length);
//     // let react_parameter   = parameter(rectangle1.width, rectangle1.length);
//     let rectangle1 = Rectangle::new(7.0, 12.5);

//     let react_area = rectangle1.area();
//     let react_parameter = rectangle1.parameter();


//     // println!("the area of a rectangle is {:#?}", react_area);
//     // println!("the parameter of a rectangle is {:#?}", react_parameter);

//     println!("the area of a rectangle is {:#?}", rectangle1.area());
//     println!("the parameter of a rectangle is {:#?}", rectangle1.parameter());


// }

// #[derive(Debug)]
// struct Rectangle{
//     width:f64,
//     length:f64,
// }

// fn area(width:f64, length:f64) -> f64{
//     width*length
// }

// fn parameter(length:f64, width:f64) -> f64{
//     2.0 * (width + length)
// }


// impl Rectangle {
//     fn new(width:f64, length:f64) -> Self{
//         if (width <= 0.0 || length <= 0.0){
//             panic!("width and lenght must be positive ");
//         }
//         Self { width, length }
//     }
//     fn area(&self) -> f64{
//         self.width*self.length

//     }
//     fn parameter(&self) -> f64{
//         2.0 * (self.width + self.length)

//     }
// }

impl PrettyPrint for Programmer {
    fn pretty_print(&self) {
        println!(
            "{{\n\t\"email\": {},\n\t\"github_repo\": {},\n\t\"blog_url\": {}\n}}",
            self.email, self.github, self.blog
        )
    }
}


#[derive(Debug)]
struct Programmer {
    email: String,
    github: String,
    blog: String,
}
fn main(){

    println!("{:?}", pg);
}