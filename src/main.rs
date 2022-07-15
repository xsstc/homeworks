fn sumall(num:&[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for i in num {  
        let sum1 = sum.checked_add(*i);
        if sum1 == None {
            return None;
        }   
        sum +=i;   
    }
   
    Some(sum)
}

fn main() {
    let num:[u32;4] = [1,2,3,4];
    let sum_num = sumall(&num);
    if let Some(v) = sum_num {
        println!("输出 ok:{:?}",v);    
    }else {
        println!("溢出 None");
    }   
  
    let over:[u32;4] = [4294967290,10,3,4];
    let sum_over = sumall(&over);
    if let Some(v) = sum_over {
        println!("输出:{:?}",v);    
    }else {
        println!("溢出 None");
    }

}
