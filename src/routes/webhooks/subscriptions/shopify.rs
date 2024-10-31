#[derive(PartialEq)]
pub enum Status {
    Active,
}

pub fn initiate_shopify_subscriptions() -> Status {
    return order_created();
}

fn order_created() -> Status {
    return Status::Active;
}
