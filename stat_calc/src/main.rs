/*
Given a list of integers, use a vector and return the mean (the average value),
median (when sorted, the value in the middle position), and mode (the value that
occurs most often; a hash map will be helpful here) of the list.
*/

use std::collections::HashMap;

fn mean(v: &Vec<i32>) -> f64 {
    let mut sum = 0;
    let mut count = 0;
    for i in v {
        sum += i;
        count += 1;
    }

    (sum as f64) / (count as f64)
}

fn median(v: &Vec<i32>) -> f64 {
    let vlen = v.len();
    if (vlen % 2) == 0 {
        println!("even");
        return ((v[vlen/2] + v[vlen/2 - 1]) as f64)/2.0;
    } else {
        println!("odd");
        return v[vlen/2] as f64;
    }
}

fn mode(v: &Vec<i32>) -> f64 {
    let mut map = HashMap::new();

    for i in v {
        let count = map.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut max_count = 0;
    let mut max_value = 0;
    for (k,v) in &map {
        if *v > max_count {
            max_value = *k;
            max_count = *v;
        }
    }

    return max_value as f64;
}

fn main() {
    let mut data = vec![11,12,10,9,9,12,11,12];

    data.sort();

    println!("{:?}",data);

    println!("mean: {}", mean(&data));
    println!("median: {}", median(&data));
    println!("mode: {}", mode(&data));
}
