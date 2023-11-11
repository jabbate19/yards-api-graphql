use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject)]
pub struct QueryServerInput {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub name_contains: Option<String>,
}

#[derive(GraphQLInputObject)]
pub struct QueryDeviceInput {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub name_contains: Option<String>,
    pub interface_has: Option<QueryInterfaceInput>,
}

#[derive(GraphQLInputObject)]
pub struct QueryInterfaceInput {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub name_contains: Option<String>,
    pub address_has: Option<QueryAddressInput>,
}

#[derive(GraphQLInputObject)]
pub struct QueryAddressInput {
    pub id: Option<i32>,
    pub static_address: Option<String>,
    pub static_address_subnet: Option<String>,
    pub iprange_has: Option<QueryIPRangeInput>,
}

#[derive(GraphQLInputObject)]
pub struct QueryIPRangeInput {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub name_contains: Option<String>,
}
