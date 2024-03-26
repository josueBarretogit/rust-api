#![allow(dead_code)]

use std::{collections::HashMap};

use helpers::calcs::is_odd;

use crate::helpers::calcs::{get_avg, get_biggest_in_hash};

mod helpers;

fn main() {


    let list_int = [3, 5, 2, 7, 3];

    let list_words  = ["hola".to_string(), "como".to_string(), "apple".to_string(), "first".to_string()];



    let median_result = median(&list_int);

    let mode_result = get_mode(&list_int); 

    let pig_latinized_words = convert_to_pig_latin(&list_words);


    println!("median is: {}", median_result);
    println!("mode is: {}", mode_result);
    println!("pig latin is: {:?}", pig_latinized_words);

    

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
            let index_1 = copy_arr.len() / 2 as usize;
            let first_middle_num_slice = &copy_arr[0..index_1];
            let second_middle_num_slice = &copy_arr[first_middle_num_slice.len()..];

            let  first_middle_num: &i32 = first_middle_num_slice.last().unwrap();
            let second_middle_num : &i32   = second_middle_num_slice.first().unwrap();


            get_avg(&[*first_middle_num, *second_middle_num])

        },
        true => {

            let middle_num   = copy_arr[0..(copy_arr.len() / 2) + 1].last().unwrap();

            *middle_num as f64
        }

    }
 
}

fn get_mode(list_integer: &[i32]) -> i32 {

    let mut values : HashMap<String, i32> = HashMap::new();

    for  current_value in list_integer.iter(){
        let current_count = values.entry(current_value.to_string()).or_insert(1);
        *current_count += 1;
    }


    get_biggest_in_hash(&values).1
}


//if first letter is consonant
//move consonant to the end and
//add ay example
//first -> irst-fay
fn convert_to_pig_latin(words: &[String]) -> Vec<String> {

    let mut coll_words: Vec<String> = words.to_vec();

    let vocals : Vec<char>  = vec!['a', 'e', 'i', 'o', 'u'];

    for word in coll_words.iter_mut() {

        let first_letter = &word.chars().next().unwrap();
        if !vocals.contains(&word.chars().next().unwrap()) {

            word.push('-');
            word.push(*first_letter);
            word.push('a');
            word.push('y');

            word.remove(0);
        } else {

            word.push('-');
            word.push('h');
            word.push('a');
            word.push('y');
        }
    }

    coll_words

}



