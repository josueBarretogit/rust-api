use std::{collections::HashMap, i32};


pub fn get_avg(arr : &[i32]) -> f64 {

    if arr.len() <= 0 {
        return  0.0;
    }

    let mut sum_elements : i32 = 0;

    for elem in arr.iter() {
        sum_elements += elem;
    }

    let avg:  f64 = (sum_elements as f64) / (arr.len() as f64);
    avg

}


pub fn is_odd(num: usize ) -> bool {
    let remainder  = num % 2;
    if  remainder >= 1 {
        return true
    }
    false
}


//get the key with the biggest number
pub fn get_biggest_in_hash(h : &HashMap<String, i32>) -> (String, i32) {

    let mut biggest = 0;
    let  key  = String::from("key");

    for key_value in h {
        if *key_value.1 > biggest {
           biggest = *key_value.1;
        }
    }

    (key, biggest)

}
