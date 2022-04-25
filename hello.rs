fn main(){
	let hello:String=String::from("hello world hi there ...");
	let hellos=hello.split(" ");
	for s in hellos{
		println!("{}",s);
	}
}
