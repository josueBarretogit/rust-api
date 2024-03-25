use std::{collections::HashMap, usize};

use helpers::calcs::is_odd;

use crate::helpers::calcs::get_avg;

mod helpers;

fn main() {

    let   list_int = [1, 3, 4, 2, 6, 5, 8, 7];


    let median_result = median(&list_int);

    println!("median is: {}", median_result);

    println!("average is {}", get_avg(&list_int));
    println!("Is odd? {}", is_odd(list_int.len()));



}


//first, sort the array
//if number of elements is odd, the median is the element in the middle
//if number of elements is even, calculate the avg of the middle elements
fn median( list_integer: &[i32] ) -> f64 {

    if list_integer.len() <= 0 {
        return 0.0;
    }

    let mut copy_arr = list_integer.to_owned();

    copy_arr.sort();

    let arr_length = copy_arr.len();
    let is_odd = is_odd(arr_length);

    match is_odd {
        false => {

            let first_middle_num_arr = &copy_arr[0..copy_arr.len() / 2];
            let second_middle_num_arr = &copy_arr[first_middle_num_arr.len()..];

            let  first_middle_num: &i32 = first_middle_num_arr.last().unwrap();
            let second_middle_num : &i32   = second_middle_num_arr.first().unwrap();

            println!("{}", first_middle_num);
            println!("{}", second_middle_num);

            get_avg(&[*first_middle_num, *second_middle_num])

        },
        true => {

            let middle_num : f64 = copy_arr[0..copy_arr.len() / 2][0].into();

            middle_num
        }

    }
 
}

fn mode_calc(list_integer: &[i32]) -> i32 {

    return list_integer[0]
}



