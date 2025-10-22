struct Rectangle{
	width:u32,
	height:u32,
}
impl Rectangle{
	fn area(&self)->u32{
		self.width*self.height
		}
		}
impl Rectangle{
	fn can_hold(&self,other:&Rectangle)->bool{
	self.width>other.width && self.height>other.height
	}
	}
	
fn main(){
let rect1=Rectangle{
	width:23,
	height:40,
	};
let rect2=Rectangle{
	width:69,
	height:55,
	};
println!("the area is {}",rect1.area());
println!("can rect1 hold rect2{}",rect1.can_hold(&rect2));
}
 
