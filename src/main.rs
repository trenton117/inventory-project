fn main() {

// Plan
// 1. Create an empty inventory (a vector of strings)
// 2. Make a function to add items to inventory
// 3. Make a function to remove items from inventory
// 4. Make a function to list all items in inventory

// create a vector that holds strings, named 'inventory'. it's a new empty vector, and mutable to add/remove things later.
    let mut inventory: Vec<String> = Vec::new();

// use the add_item fn to add strings 'Sword' and 'Shield' to the vector. IMPORTANT: CONVERTS '&str' to 'String'.
    add_item(&mut inventory, "Sword".to_string());
    add_item(&mut inventory, "Shield".to_string());

// print the contents of the 'inventory' vector
    show_inventory(&inventory);

// remove_item fn to remove 'Sword', and then remove it again. (test to show that it's no longer there and can't be removed twice).
    remove_item(&mut inventory, "Sword".to_string());

    remove_item(&mut inventory, "Sword".to_string());

// Print the contents of 'inventory' vector again to show 'Sword' is no longer there.
    show_inventory(&inventory);
}

// add_item fn. it adds items to the inventory vector.
fn add_item(inventory: &mut Vec<String>, item: String) {
    inventory.push(item);
}

// remove_item fn. it removes items from the inventory vector. still need to study/understand this.
// acts upon the 'inventory' vector and modifies it (&mut). takes 'item' argument, which is a string.
// UPDATE: "mutably borrows" the inventory, taking ownership of the 'item' string, then searches for and removes it by index if found.
fn remove_item(inventory: &mut Vec<String>, item: String) {

// This if statement is where my head starts to hurt.
// Checks to see if the 'item' argument exists in the vector, and where. If it exists, remove that index of the vector.

// UPDATE: .position() returns Some(index) if the item is found, or None if not.
// The if-let unpacks the index as 'pos' and removes the item at that index.
    if let Option::Some(pos) = inventory.iter().position(|x| *x == item) {
        inventory.remove(pos);
    } else {
        println!("Item {} not found.", item);
    }
        
}

// show_inventory fn, just prints the contents of the 'inventory' vector, uses below 'print_item' fn.
fn show_inventory(inventory: &Vec<String>) {
    println!("Inventory:");
    for item in inventory {
        print_item(item);
    }
}

// fn to simply list the items (Strings) in a given argument.
fn print_item(item: &String) {
    println!("{}", item);
}

