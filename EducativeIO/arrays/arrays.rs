fn main() {
    //define an array of size 4
    let arr:[i32;4] = [1, 2, 3, 4]; 
    //print the first element of array
    println!("The first value of array is {}", arr[0]);
    // initialize an array of size 4 with 0
    let arr1 = [0; 4]; 
    //print the first element of array
    println!("The first value of array is {}", arr1[0]);

    //define an array of size 4
    let arr2:[i32;4] = [1, 2, 3, 4]; 
    //print the first element of array
    println!("The first value of array is {}", arr2[0]);
    // initialize an array of size 4 with 0
    let arr3 = [0; 4]; 
    //print the first element of array
    println!("The first value of array is {}", arr3[0]);

    //define an array of size 4
    let arr4:[i32;4] = [1, 2, 3, 4]; 
    //Using debug trait
    println!("\nPrint using a debug trait");
    println!("Array: {:?}", arr4);

    //define an array of size 4
    let arr5:[i32;4] = [1, 2, 3, 4]; 
    // print the length of array
    println!("Length of array: {}", arr5.len());

    //define an array of size 4
    let arr6:[i32;4] = [1, 2, 3, 4]; 
    //define the slice
    let slice_array1:&[i32] = &arr6;
    let slice_array2:&[i32] = &arr6[0..2];
    // print the slice of an array
    println!("Slice of an array: {:?}", slice_array1);
    println!("Slice of an array: {:?}", slice_array2);
}