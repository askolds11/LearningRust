use std::collections::HashMap;

/*
Given a list of integers, use a vector and return
the median (when sorted, the value in the middle position) and
mode (the value that occurs most often; a hash map will be helpful here) of the list.
*/
fn main() {
    // do not make it mutable, just because.
    let integers: Vec<i32> = vec![1, 2, 3];

    if integers.len() == 0 {
        println!("No median or mode - empty list!");
        return;
    }

    // sort
    let mut integers_sorted = integers.clone();
    integers_sorted.sort_unstable();

    // even - position of first element, e.g. 4 elements - position is 1 (0-indexed)
    // uneven - position of middle element, e.g. 5 elements - position is 2 (0-indexed)
    let median_position = integers_sorted.len() / 2;

    let median = if integers_sorted.len() % 2 == 0 {
        (
            integers_sorted[median_position - 1] +
            integers_sorted[median_position]
        ) as f64 / 2.0

    } else {
        integers_sorted[median_position] as f64
    };
    println!("The median is {median}");

    let mut occurences: HashMap<i32, i32> = HashMap::new();
    for i in integers {
        occurences.entry(i)
            .and_modify(|count| { *count += 1 })
            .or_insert(1);
    }
    

    let mut max: i32 = 0;
    let mut maxes: Vec<i32> = Vec::new();
    for occurence in occurences {
        if occurence.1 > max {
            maxes.clear();
            maxes.push(occurence.0);
            max = occurence.1;
        } else if occurence.1 == max {
            maxes.push(occurence.0);
        }
    }

if maxes.len() == 1 {
    println!("The mode is {maxes:?}")  ;     
} else {
    println!("The modes are {maxes:?}");
}
}
