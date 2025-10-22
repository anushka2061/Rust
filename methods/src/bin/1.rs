#[derive(Debug)]
struct Rectangle{
	width:u32,
	height:u32,
	}
impl Rectangle{
	fn square(size:u32)->Self{
		Self{
			width:size,
			height:size,
		}
	}
}
fn main(){
let rect1=Rectangle{
	width:30,
	height:10,
	};
let sq=Rectangle::square(30);
println!("the value is {:?}",sq);
}
