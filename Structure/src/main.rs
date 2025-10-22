fn main() {
    struct User{
    	active:bool,
    	username:String,
    	email:String,
    	sign_in_count:u64,
    }
   let user1=User{
   active:true,
   username:String::from("anushka"),
   email:String::from("something@gmail.com"),
   sign_in_count:1,
};
println!("the name is {}",user1.username);
}
