fn main() {
    let arr1: [i32; 4] = [10, 20, 30, 40];

    let arr2 = [10.4, 20.7, 30.5, 40.2];

    let arr3: [i32; 8] = [-1; 8];

    println!("arr1 = {:?}", arr1);
    println!("Length of arr1 = {}", arr1.len());

    println!("arr2 = {:?}", arr2);
    println!("Length of arr2 = {}", arr2.len());

    println!("arr3 = {:?}", arr3);
    println!("Length of arr3 = {}", arr3.len());
}