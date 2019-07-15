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
        return ((v[vlen/2] + v[vlen/2 - 1]) as f64)/2.0;
    } else {
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
    let mut data1 = vec![-10, 0, -6, 6, 3, -8, 2, -8, -8, -2];
    let mut data2 = vec![4, 2, -6, -3, -9, 2, 0, 3, 5, 2];
    let mut data3 = vec![0, 9, -1, 8, 3, -6, 1, -7, 7, -6];


    data1.sort();
    data2.sort();
    data3.sort();

    println!("{:?}", data3);

    assert_eq!(mean(&data1), -3.1);
    assert_eq!(median(&data1), -4.0);
    assert_eq!(mode(&data1), -8.0);

    assert_eq!(mean(&data2), 0.0);
    assert_eq!(median(&data2), 2.0);
    assert_eq!(mode(&data2), 2.0);

    assert_eq!(mean(&data3), 0.8);
    assert_eq!(median(&data3), 0.5);
    assert_eq!(mode(&data3), -6.0);

    println!("data1");
    println!("mean: {}", mean(&data1));
    println!("median: {}", median(&data1));
    println!("mode: {}", mode(&data1));
    println!();
}
