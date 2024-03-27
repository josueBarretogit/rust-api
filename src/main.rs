use crate::agrega::agregate::{Summary, Tweet};

mod agrega;

fn main() {
    let list_int = [1 , 2, 10];
    let other_list : [i64; 3] = [3,4,23];

    println!("the largest is: {}", find_largest::<i32>(&list_int));
    println!("the largest is: {}", find_largest::<i64>(&other_list));

    let tweet : Tweet = Tweet { username: "as".to_string(), content: "asd".to_string() , retweet: true, reply: false };

    println!("a twwet: {}" , tweet.summarize(String::from("as")) )
}

fn find_largest<T : std::cmp::PartialOrd>(arr: &[T]) -> &T {

    let mut biggest = &arr[0];

    for ele in arr {
        if ele > biggest {
            biggest = ele
        }
    }

    biggest
}
