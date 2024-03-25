use std::{collections::HashMap, usize};

mod helpers;

fn main() {

    let  list_int = [1,4,5,5,4,7, 152];


    let median_result = median(&list_int);

    println!("median is: {}", median_result);
    println!("median is: {:?}", list_int);

    println!("mode is: {}", mode_calc(&list_int));

}


//first, sort the array
//second, sum all elemnts
//if number of elements is odd, the median is the element in the middle
//if number of elements is even, calculate the avg of the middle elements
fn median( list_integer: &[i32] ) -> usize {

    let mut co = list_integer.to_owned();

    co.sort();

    for elem in co.iter().rev() {

        println!("sorted: {}", elem);

    }
    return 1 
}

fn mode_calc(list_integer: &[i32]) -> i32 {

    return list_integer[0]
}



