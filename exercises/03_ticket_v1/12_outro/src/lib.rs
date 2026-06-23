// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u8,
    unit_price: u16,
}
pub fn validate_product_name(product_name: &str) {
    if product_name.is_empty() {
        panic!("empty product name");
    }
    if product_name.len() > 300 {
        panic!("too many characters for product name");
    }
}
pub fn validate_quantity(quantity: u8) {
    if quantity == 0 {
        panic!("quantity must be > 0");
    }
}

pub fn validate_unit_price(unit_price: u16) {
    if unit_price == 0 {
        panic!("unit_price must be > 0");  // ← 应该是 " must be > 0"
    }
}

impl Order {
    pub fn new(product_name: String, quantity: u8, unit_price: u16) -> Self {
        validate_product_name(&product_name);
        validate_quantity(quantity);
        validate_unit_price(unit_price);
        Order {
            product_name,
            quantity,
            unit_price,
        }
    }
    pub fn set_product_name(&mut self, new_product_name: String) {
        self.product_name = new_product_name;
    }
    pub fn product_name(&self) -> &String {
        return &self.product_name;
    }

    pub fn set_quantity(&mut self, new_quantity: u8) {
        self.quantity = new_quantity;
    }
    pub fn quantity(&self) -> &u8 {
        return &self.quantity;
    }

    pub fn set_unit_price(&mut self, new_price: u16) {
        self.unit_price = new_price;
    }
    pub fn unit_price(&self) -> &u16 {
        return &self.unit_price;
    }
    pub fn total(&self) -> u32 {
        self.quantity as u32 * self.unit_price as u32
    }
}
