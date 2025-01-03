use std::collections::HashMap;
use std::io;

fn main() {
    // Menu and prices
    let menu: HashMap<&str, (&str, f64)> = HashMap::from([
        ("P", ("Poundo Yam/Edinkaiko Soup", 3200.0)),
        ("F", ("Fried Rice & Chicken", 3000.0)),
        ("A", ("Amala & Ewedu Soup", 2500.0)),
        ("E", ("Eba & Egusi Soup", 2000.0)),
        ("W", ("White Rice & Stew", 2500.0)),
    ]);

    println!("Menu:");
    for (key, &(name, price)) in &menu {
        println!("{} = {} - N{:.2}", key, name, price);
    }

    let mut total_cost = 0.0;

    loop {
        // Input food type
        let mut food_input = String::new();
        println!("Enter the food type (P, F, A, E, W) or 'done' to finish:");
        io::stdin()
            .read_line(&mut food_input)
            .expect("Failed to read input");
        let food_input = food_input.trim();

        if food_input.eq_ignore_ascii_case("done") {
            break;
        }

        if let Some(&(name, price)) = menu.get(food_input) {
            // Input quantity
            let mut quantity_input = String::new();
            println!("Enter the quantity of {}:", name);
            io::stdin()
                .read_line(&mut quantity_input)
                .expect("Failed to read input");
            let quantity: u32 = quantity_input.trim().parse().unwrap_or(0);

            // Calculate and add to total cost
            let cost = price * quantity as f64;
            total_cost += cost;
            println!(
                "Added {} x {} (N{:.2}) = N{:.2} to your order.",
                quantity, name, price, cost
            );
        } else {
            println!("Invalid food type. Please select from the menu.");
        }
    }

    // Apply discount if total cost > N10,000
    if total_cost > 10_000.0 {
        let discount = total_cost * 0.05;
        total_cost -= discount;
        println!("A 5% discount of N{:.2} has been applied.", discount);
    }

    println!("Total cost of your order: N{:.2}", total_cost);
}
