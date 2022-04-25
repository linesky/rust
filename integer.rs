fn main(){
	let group1:[f32;6]=[-2.20,-1.00,0.00,1.10,2.00,3.00];
	let mut group2n=0;
	let mut group3n=0;
	let mut group2:[f32;6]=[0.00;6];
	let mut group3:[f32;6]=[0.00;6];
	let mut n:i32=0;
	let mut nn:f32=0.00;
	for a in 0..group1.len(){
		n=group1[a].floor() as i32;
		nn=n as f32;
		if group1[a]==nn {
			group2[group2n]=group1[a];
			group2n=group2n+1;
		}else{
			group3[group3n]=group1[a];
			group3n=group3n+1;
		}
	}
	println!("group : ");
	println!("{:?}",group1);
	println!("integer group : ");
	for ab in 0..group2n{
		println!("{}",group2[ab]);
	}
	println!("not integer group : ");
	for ac in 0..group3n{
		println!("{}",group3[ac]);
	}

}
