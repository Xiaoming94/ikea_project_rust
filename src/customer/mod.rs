use warehouse::Warehouse

pub struct Customer {
    m_name: String,
}

pub fn create_customer (name: String) -> Customer {
    Customer { m_name: name }
}

//pub fn customer_send_request(customer: Customer, warehouse: Warehouse)