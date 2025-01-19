struct Laptop {
    brand: String,
    price: u32,
    quantity: u32,
}

impl Laptop {
    fn calculate_cost(&self, num_purchased: u32) -> u32 {
        self.price * num_purchased
    }
}

fn main() {
    let hp = Laptop {
        brand: String::from("HP"),
        price: 650_000,
        quantity: 10,
    };

    let ibm = Laptop {
        brand: String::from("IBM"),
        price: 755_000,
        quantity: 6,
    };

    let toshiba = Laptop {
        brand: String::from("Toshiba"),
        price: 550_000,
        quantity: 10,
    };

    let dell = Laptop {
        brand: String::from("Dell"),
        price: 850_000,
        quantity: 4,
    };

    let laptops = vec![hp, ibm, toshiba, dell];
    let mut total_cost = 0;

    for laptop in &laptops {
        total_cost += laptop.calculate_cost(3); // Customer buys 3 from each brand
    }

    println!("The total cost of purchasing 3 laptops from each brand is: ₦{}", total_cost);
}
