fn main(){
	let group1:[i32;6]=[-2,-1,0,1,2,3];
	let mut negn=0;
	let mut nnegn=0;
	let aa:i32=0;
	let mut negative:[i32;6]=[0;6];
	let mut notnegative:[i32;6]=[0;6];
	for a in 0..group1.len(){
		if group1[a]<aa {
			negative[negn]=group1[a];
			negn=negn+1;
		}else{
			notnegative[nnegn]=group1[a];
			nnegn=nnegn+1;
		}
	}
	println!("group : ");
	println!("{:?}",group1);
	println!("negative group : ");
	for ab in 0..negn{
		println!("{}",negative[ab]);
	}
	println!("negative group : ");
	for ac in 0..nnegn{
		println!("{}",notnegative[ac]);
	}

}
