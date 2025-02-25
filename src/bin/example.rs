use true_crab_utils::data_structures::array;

fn main() {
    let mut arr = vec![2, 4, 10, 5, 15, 3];

    array::reverse(&mut arr);
    array::insert(&mut arr, 100, 3);

    println!("Modified array: {arr:?}");
    println!(
        "Merged and sorted: {:?}",
        array::merge_and_sort(&arr, &[9, 1])
    );
}
