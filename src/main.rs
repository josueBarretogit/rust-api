

fn main() {
    let list_int = [1 , 2, 10];

    println!("sum of elements is: {}", sum_elem_arrays(&list_int));
}

//is 4
fn fibonnacci_rust(n : usize) -> usize {
    if n <= 0 {
        return 0
    }
    if n == 1 {
        return 1
    }
    fibonnacci_rust(n - 1 ) + fibonnacci_rust(n - 2)
}


//
//Sum all elements of array recursively
fn sum_elem_arrays(arr : &[usize]) -> usize {
    if arr.len() <= 1 {
        return arr[0]
    }

    sum_elem_arrays(arr)

}


