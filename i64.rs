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
println!("{}",toString(add(100,200)));
println!("{}",toString(sub(200,100)));
println!("{}",toString(mul(200,100)));
println!("{}",toString(div(200,5)));
}
