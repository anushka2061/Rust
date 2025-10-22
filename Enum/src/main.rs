#[derive(Debug)]
enum IPAddrKind{
	V4,
	V6,
	}
struct IpAddr{
	kind:IPAddrKind,
	address:String,
	}
impl IpAddr{
fn display(&self)->String{
	format!("{:?}: {}", self.kind, self.address)
		}
	}

fn main() {
	let home=IpAddr{
	kind:IPAddrKind::V4,
	address:String::from("127.0.0.1")
	,
	};
	let loopback=IpAddr{
	kind:IPAddrKind::V6,
	address:String::from("::1"),
	};
    println!("{:?}",home.display());
    println!("{:?}",loopback.display());
    
}
