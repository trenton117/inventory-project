fn main() {

// Plan
// 1. Create an empty inventory (a vector of strings)
// 2. Make a function to add items to inventory
// 3. Make a function to remove items from inventory
// 4. Make a function to list all items in inventory

    let mut inventory: Vec<String> = Vec::new();

    add_item(&mut inventory, "Sword".to_string());
    add_item(&mut inventory, "Shield".to_string());

    show_inventory(&inventory);

    remove_item(&mut inventory, "Sword".to_string());

    show_inventory(&inventory);
}

fn add_item(inventory: &mut Vec<String>, item: String) {
    inventory.push(item);
}

fn remove_item(inventory: &mut Vec<String>, item: String) {
    if let Some(pos) = inventory.iter().position(|x| *x == item) {
        inventory.remove(pos);
    } else {
        println!("Item {} not found in inventory.", item);
    }
}

fn show_inventory(inventory: &Vec<String>) {
    println!("Inventory:");
    for item in inventory {
        print_item(item);
    }
}

fn print_item(item: &String) {
    println!("{}", item);
}

