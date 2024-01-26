fn main() {
    
    let arr_a: [u8;3]=[1,2,3];
    let arr_b: [u8;5]=[1,2,3,4,5];

    println!("index {}, length {}",arr_a[0], arr_b.len());

    println!("{:?}", arr_b);

    //tuple
    let tuple: (u8,bool,f32)=(5, true,2.1);
    let tuple2 = (3,5);


    println!("first {}, second {}, third {}", tuple.0, tuple.1, tuple.2);
    println!("{:?}", tuple2);

    //destucture tuple
    let (a,b,c)= tuple;
    println!("first {}, second {}, third {}", a,b,c);
    
    //function
    println!("is_even (2) : {}", is_even(2));

    //mutable variable declaration
    let mut var_a =10;
    println!("var_a : {}", var_a);

    var_a=15;

    println!("var_a : {}", var_a);


    //Array + slices

    //Take Ownership
    let mut original_array = [1, 2, 3, 4, 5];
    println!("Original array: {:?}", original_array); 

    let mut owned_slice: [i32; 3] = original_array[0..3].try_into().unwrap(); // Taking ownership of a slice (actually creating a new array)
    owned_slice[0] = 10; // Modify the owned slice, Changes made to the owned slice won't be reflected in the original array
    original_array[1]=15;
    println!("update slice [0]=10");
    println!("update original_array[1]=15; // can update original_array because is not borrowed");
    println!("Original array-takeownerwship]: {:?}", original_array);   
    println!("Owned slice-takeownership: {:?}", owned_slice);
    println!("Original array-takeownerwship]: {:?}", original_array); 

    

    //Borrowing   
    println!("");
    original_array = [1, 2, 3, 4, 5];
    println!("Original array: {:?}", original_array); 
    let owned_slice1 = &mut original_array[0..3];   // Creating a mutable reference to a slice (pointer)
    
    owned_slice1[0] = 10;
     //original_array[1]=15;
    println!("update slice [0]=10");
    println!("original_array[1]=15;  cannot updated original_array it has already borrowed.  compilation error");
    println!("Modified slice-borrowing: {:?}", owned_slice1); // Accessing the modified slice through the reference
    println!("Original array-borrowing: {:?}", original_array);  // Changes to the slice are reflected in the original array  


    //String
    let str="string sample";
    let str_object = String::from("string object sample");




}


pub fn is_even(num :u8) -> bool {
    let digit = num %2;

    digit==0 //return 
}
