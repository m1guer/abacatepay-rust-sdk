# AbacatePay Rust SDK

A Rust SDK for integrating with the AbacatePay payment platform

## Features

- Create one-time billings with PIX payment method
- List existing billings
- Create PIX QR code charges
- Check PIX payment status
- Simulate PIX payments (for testing)

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

### Creating a PIX Charge

```rust
use abacatepay_rust_sdk::{AbacatePay, CustomerMetadata};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AbacatePay::new("api_key".to_string());

    // Create a basic PIX charge
    let pix_charge = client
        .create_pix_charge()
        .amount(100.0)
        .description(Some("Payment for services".to_string()))
        .expires_in(Some(3600)) // Expires in 1 hour
        .build()
        .await?;

    println!("Created PIX charge: {:?}", pix_charge);
    println!("QR Code URL: {}", pix_charge.qrcode_image_url);
    println!("PIX Copy-and-paste: {}", pix_charge.brcode);

    // Create a PIX charge with customer information
    let pix_charge_with_customer = client
        .create_pix_charge()
        .amount(150.0)
        .description(Some("Product purchase".to_string()))
        .customer(Some(CustomerMetadata {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            tax_id: "123.456.789-00".to_string(),
            cellphone: "5511999999999".to_string(),
        }))
        .build()
        .await?;

    println!("Created PIX charge with customer: {:?}", pix_charge_with_customer);
    
    Ok(())
}
```

### Checking PIX Payment Status

```rust
use abacatepay_rust_sdk::AbacatePay;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AbacatePay::new("api_key".to_string());
    
    // Check status of a PIX payment
    let payment_status = client
        .check_pix_status("pix-charge-id".to_string())
        .build()
        .await?;
    
    println!("Payment status: {:?}", payment_status.status);
    
    Ok(())
}
```

### Simulating a PIX Payment (Testing Only)

```rust
use abacatepay_rust_sdk::AbacatePay;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AbacatePay::new("api_key".to_string());

    // Create a PIX charge first
    let pix_charge = client
        .create_pix_charge()
        .amount(100.0)
        .build()
        .await?;
    
    // Simulate a payment for the created charge
    let payment_result = client
        .create_simulate_pix_payment(pix_charge.id)
        .build()
        .await?;
    
    println!("Payment simulation result: {:?}", payment_result);
    
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

### Creating a PIX Charge

```rust
use abacatepay_rust_sdk::{AbacatePay, CustomerMetadata};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AbacatePay::new("api_key".to_string());

    // Create a basic PIX charge
    let pix_charge = client
        .create_pix_charge()
        .amount(100.0)
        .description(Some("Payment for services".to_string()))
        .expires_in(Some(3600)) // Expires in 1 hour
        .build()
        .await?;

    println!("Created PIX charge: {:?}", pix_charge);
    println!("QR Code URL: {}", pix_charge.qrcode_image_url);
    println!("PIX Copy-and-paste: {}", pix_charge.brcode);

    // Create a PIX charge with customer information
    let pix_charge_with_customer = client
        .create_pix_charge()
        .amount(150.0)
        .description(Some("Product purchase".to_string()))
        .customer(Some(CustomerMetadata {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            tax_id: "123.456.789-00".to_string(),
            cellphone: "5511999999999".to_string(),
        }))
        .build()
        .await?;

    println!("Created PIX charge with customer: {:?}", pix_charge_with_customer);
    
    Ok(())
}
```

### Simulating a PIX Payment (Testing Only)

```rust
use abacatepay_rust_sdk::AbacatePay;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AbacatePay::new("api_key".to_string());

    // Create a PIX charge first
    let pix_charge = client
        .create_pix_charge()
        .amount(100.0)
        .build()
        .await?;
    
    // Simulate a payment for the created charge
    let payment_result = client
        .create_simulate_pix_payment(pix_charge.id)
        .build()
        .await?;
    
    println!("Payment simulation result: {:?}", payment_result);
    
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

### PIX Charge Creation Options

The PIX charge builder supports the following methods:

- `amount(f64)`: Set the charge amount in BRL
- `expires_in(Option<u64>)`: Set the expiration time in seconds (optional)
- `description(Option<String>)`: Add a description for the charge (optional)
- `customer(Option<CustomerMetadata>)`: Add customer information (optional)

### PIX Status Check Options

The PIX status check builder supports the following methods:

- `id(String)`: Set or change the PIX charge ID to check status for

### PIX Payment Simulation

The PIX payment simulation builder supports the following methods:

- `id(String)`: Set or change the PIX charge ID to simulate payment for

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

#### CustomerMetadata

```rust
pub struct CustomerMetadata {
    pub name: String,
    pub email: String,
    pub tax_id: String,
    pub cellphone: String,
}
```

#### PixChargeData

The `PixChargeData` structure contains information about a created PIX charge, including:

- `id`: The unique identifier for the PIX charge
- `status`: The current status of the charge
- `qrcode_image_url`: URL to the QR code image that can be scanned for payment
- `brcode`: The PIX copy-and-paste code
- `amount`: The charge amount
- `created_at`: When the charge was created
- `expires_at`: When the charge expires (if set)
- `paid_at`: When the charge was paid (if applicable)

#### CheckPixStatusData

The `CheckPixStatusData` structure contains information about the status of a PIX payment, including:

- `status`: The current status of the payment (e.g., PENDING, PAID, EXPIRED)
- `paid_at`: When the payment was made (if applicable)
```
