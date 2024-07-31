mod linked_list;

use linked_list::VLinkedList;

fn main() {
    let mut list: VLinkedList<i32> = VLinkedList::new();

    // insert operations
    list.add_to_start(1);
    list.add_to_start(2);
    list.add_to_start(3);
    list.add_to_end(4);
    list.add_to_end(5);
    list.add_to_end(6);

    // make a vector
    let vector = list.to_vector();

    // display results
    println!("{:?}", vector);
    println!("{:?}", list);

    // removal operations
    list.remove_from_start();
    list.remove_from_start();
    list.remove_from_end();
    list.remove_from_end();
    list.remove_from_end();
    list.remove_from_end();

    // display results
    println!("{:?}", list);
}
