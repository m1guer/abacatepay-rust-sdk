# AbacatePay Rust SDK

A Rust SDK for integrating with the AbacatePay payment platform

## Features

- Create one-time billings with PIX payment method
- List existing billings

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
abacatepay-rust-sdk = "0.1.2"
```

## Usage

### Creating a Client

```rust
use abacatepay_rust_sdk::AbacatePay;

let client = AbacatePay::new("your_api_key".to_string());
```

### Creating a Billing

```rust
use abacatepay_rust_sdk::{AbacatePay, BillingKind, BillingMethods, CreateBillingProduct};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AbacatePay::new("api_key".to_string());

    let billing = client
        .create_billing()
        .frequency(BillingKind::OneTime)
        .method(BillingMethods::Pix)
        .product(CreateBillingProduct {
            external_id: "123".to_string(),
            name: "Product".to_string(),
            quantity: 1,
            price: 100.0,
            description: Some("Description".to_string()),
        })
        .return_url("http://localhost:3000/".to_string())
        .completion_url("http://localhost:3000/".to_string())
        .build()
        .await?;

    println!("Created billing: {:?}", billing);
    Ok(())
}
```

### Listing Billings

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AbacatePay::new("api_key".to_string());

    let billings = client.list_billings().await?;
    println!("All billings: {:?}", billings);

    Ok(())
}
```

## API Reference

### Billing Creation Options

The billing builder supports the following methods:

- `frequency(BillingKind)`: Set the billing frequency (currently supports `OneTime`)
- `method(BillingMethods)`: Add a payment method (currently supports `Pix`)
- `product(CreateBillingProduct)`: Add a product to the billing
- `return_url(String)`: Set the return URL for the billing
- `completion_url(String)`: Set the completion URL for the billing
- `customer_id(String)`: Set an optional customer ID

### Data Types

#### BillingStatus

```rust
pub enum BillingStatus {
    PENDING,
    EXPIRED,
    CANCELLED,
    PAID,
    REFUNDED,
}
```

#### BillingMethods

```rust
pub enum BillingMethods {
    Pix,
}
```

#### CreateBillingProduct

```rust
pub struct CreateBillingProduct {
    pub external_id: String,
    pub name: String,
    pub quantity: i64,
    pub price: f64,
    pub description: Option<String>,
}
```
