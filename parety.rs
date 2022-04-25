fn main(){
	let group1:[i32;6]=[-2,-1,0,1,2,3];
	let mut paretyn=0;
	let mut nparetyn=0;
	let mut parety:[i32;6]=[0;6];
	let mut notparety:[i32;6]=[0;6];
	for a in 0..group1.len(){
		if group1[a]/2*2==group1[a] {
			parety[paretyn]=group1[a];
			paretyn=paretyn+1;
		}else{
			notparety[nparetyn]=group1[a];
			nparetyn=nparetyn+1;
		}
	}
	println!("group : ");
	println!("{:?}",group1);
	println!("parety group : ");
	for ab in 0..paretyn{
		println!("{}",parety[ab]);
	}
	println!("not parety group : ");
	for ac in 0..nparetyn{
		println!("{}",notparety[ac]);
	}

}
