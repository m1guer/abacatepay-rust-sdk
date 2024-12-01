use abacatepay_rust_sdk::{
    AbacatePay, BillingKind, BillingMethods, CreateBillingData, CreateBillingProduct,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = AbacatePay::new("abc_dev_MUWd52PQja5UBhNTx2AtZRLQ".to_string());

    // Create a billing
    let billing_data = CreateBillingData {
        frequency: BillingKind::OneTime,
        methods: vec![BillingMethods::Pix],
        products: vec![CreateBillingProduct {
            external_id: "123".to_string(),
            name: "Product".to_string(),
            quantity: 1,
            price: 100.0,
            description: Some("Description".to_string()),
        }],
        return_url: "http://localhost:3000/".to_string(),
        completion_url: "http://localhost:3000/".to_string(),
        customer_id: None,
    };

    let billing = client.create_billing(billing_data).await?;
    println!("Created billing: {:?}", billing);

    // List billings
    let billings = client.list_billings().await?;
    println!("All billings: {:?}", billings);

    Ok(())
}
