use std::time::Instant;

fn generate_subarrays_loop(arr: &[i8]) -> Vec<Vec<i8>> {
    let mut subarrays = Vec::new();

    for i in 0..arr.len() {
        for j in i..arr.len() {
            let subarray = arr[i..=j].to_vec();
            subarrays.push(subarray);
        }
    }

    return subarrays;
}

fn generate_subarrays_recursive(
    arr: &[i8],
    start: usize,
    end: usize,
    subarrays: &mut Vec<Vec<i8>>,
) {
    if end == arr.len() {
        return;
    }

    if start > end {
        generate_subarrays_recursive(arr, 0, end + 1, subarrays);
    } else {
        let subarray = arr[start..=end].to_vec();
        subarrays.push(subarray);
        generate_subarrays_recursive(arr, start + 1, end, subarrays);
    }
}

fn main() {
    let arr = [7, 6, 3, 4, 8, 9, 1, 3, 2, 2];
    // let arr = [0; 20];
    let mut start_time = Instant::now();
    let loop_subarrays = generate_subarrays_loop(&arr);
    let loop_elapsed_time = start_time.elapsed();

    start_time = Instant::now();
    let mut recursive_subarrays = Vec::new();
    generate_subarrays_recursive(&arr, 0, 0, &mut recursive_subarrays);
    let recursive_elapsed_time = start_time.elapsed();

    println!("Original array: {:?}", arr);
    println!("All subarrays:");
    for subarray in recursive_subarrays {
        println!("{:?}", subarray);
    }
    println!("Elapsed time (loop): {:?}", loop_elapsed_time);
    println!("Elapsed time (recursive): {:?}", recursive_elapsed_time)
}
