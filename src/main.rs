fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest_item = &list[0];

    for item in list {
        if item > largest_item {
            largest_item = item;
        }
    };
    largest_item

}


fn main() {
    let number_list = vec![1,23,45,66,90590,123,55,878];
    let letter_list = vec!['a','m','s','d'];

    let _number = largest(&number_list);

    let _letter = largest(&letter_list);

    println!("The largest number is {_number}.");
    println!("The largest letter is {_letter}.");
}
