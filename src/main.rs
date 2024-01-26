fn main() {
    
    let arr_a: [u8;3]=[1,2,3];
    let arr_b: [u8;5]=[1,2,3,4,5];

    println!("index {}, length {}",arr_a[0], arr_b.len());

    println!("{:?}", arr_b);


    let tuple: (u8,bool,f32)=(5, true,2.1);
    let tuple2 = (3,5);


    println!("first {}, second {}, third {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple2);


    let (a,b,c)= tuple;
    println!("first {}, second {}, third {}", a,b,c);
    

    println!("is_even (2) : {}", is_even(2))


}


pub fn is_even(num :u8) -> bool {
    let digit = num %2;

    digit==0
}
