fn toString(a:i64)->String{
		return a.to_string();
}
fn add(a:i64,b:i64)->i64{
	return a+b;
}
fn sub(a:i64,b:i64)->i64{
	return a-b;
}
fn div(a:i64,b:i64)->i64{
	return a/b;
}
fn mul(a:i64,b:i64)->i64{
	return a*b;
}
fn main(){
	let a:i64=1234567890;
	let b:i64=10000;
	let mut c:i64=0;
	let mut cc:String=String::from("");
	c=add(a,b);
	println!("sum: {}",c);
	c=sub(a,b);
	println!("sub: {}",c);
	c=mul(a,b);
	println!("mul: {}",c);
	c=div(a,b);
	println!("div: {}",c);
	c=123456789012345678;
	println!("to string: {}",toString(c));
}
