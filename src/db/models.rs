use chrono::NaiveDateTime;
use diesel::sql_types;
use diesel::{deserialize::FromSqlRow, prelude::*};
use diesel_derive_enum::DbEnum;
use juniper::{
    serde::{Deserialize, Serialize},
    GraphQLObject,
};

#[derive(
    Identifiable, Serialize, Deserialize, Queryable, PartialEq, Clone, Debug, GraphQLObject,
)]
#[diesel(table_name = super::db::keypermissions)]
#[diesel(primary_key(id))]
pub struct KeyPermissions {
    pub id: i32,
    pub keyid: i32,
    pub permission: String,
}

#[derive(
    Identifiable, Serialize, Deserialize, Queryable, PartialEq, Clone, Debug, GraphQLObject,
)]
#[diesel(table_name = super::db::logs)]
#[diesel(primary_key(id))]
pub struct Logs {
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub message: String,
}

#[derive(
    Identifiable, Serialize, Deserialize, Queryable, PartialEq, Clone, Debug, GraphQLObject,
)]
#[diesel(table_name = super::db::device)]
#[diesel(primary_key(id))]
pub struct Device {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub owner: String,
    #[diesel(sql_type = Text)]
    pub comments: String,
    #[diesel(sql_type = Text)]
    pub group: String,
    #[diesel(sql_type = Bool)]
    pub group_edit: bool,
}

#[derive(
    Identifiable,
    Serialize,
    Deserialize,
    Queryable,
    Associations,
    PartialEq,
    Clone,
    Debug,
    GraphQLObject,
)]
#[diesel(table_name = super::db::interface)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Device, foreign_key = deviceid))]
pub struct Interface {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub macaddr: String,
    #[diesel(sql_type = Integer)]
    pub deviceid: i32,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub comments: Option<String>,
}

#[derive(juniper::GraphQLEnum, DbEnum, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[ExistingTypePath = "crate::db::db::sql_types::Iptype"]
#[DbValueStyle = "PascalCase"]
pub enum IPType {
    Static,
    Dynamic,
}

#[derive(
    Identifiable,
    Serialize,
    Deserialize,
    Queryable,
    Associations,
    PartialEq,
    Clone,
    Debug,
    GraphQLObject,
)]
#[diesel(table_name = super::db::address)]
#[diesel(primary_key(id))]
#[diesel(belongs_to(Interface, foreign_key = interfaceid))]
#[diesel(belongs_to(IPRange, foreign_key = iprangeid))]
pub struct Address {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Integer)]
    pub interfaceid: i32,
    #[diesel(sql_type = Integer)]
    pub iprangeid: i32,
    #[diesel(sql_type = Nullable<Iptype>)]
    pub iptype: Option<IPType>,
}

#[derive(
    Identifiable, Serialize, Deserialize, Queryable, PartialEq, Clone, Debug, GraphQLObject,
)]
#[diesel(table_name = super::db::staticaddress)]
#[diesel(primary_key(addressid))]
#[diesel(belongs_to(Address))]
pub struct StaticAddress {
    #[diesel(sql_type = Integer)]
    pub addressid: i32,
    #[diesel(sql_type = String)]
    pub ipaddr: String,
}

#[derive(juniper::GraphQLEnum, DbEnum, Serialize, Deserialize, PartialEq, Clone, Debug)]
#[ExistingTypePath = "crate::db::db::sql_types::Ipversion"]
#[DbValueStyle = "PascalCase"]
pub enum IPVersion {
    V4,
    V6,
}

#[derive(
    Identifiable, Serialize, Deserialize, Queryable, PartialEq, Clone, Debug, GraphQLObject,
)]
#[diesel(table_name = super::db::iprange)]
#[diesel(primary_key(id))]
pub struct IPRange {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Ipversion)]
    pub ipversion: IPVersion,
    #[diesel(sql_type = Text)]
    pub networkid: String,
    #[diesel(sql_type = Integer)]
    pub cidr: i32,
    #[diesel(sql_type = Text)]
    pub description: String,
    pub vlan: i32,
    pub gateway: String,
    pub default_dns: String,
    pub dns_domain: String,
    pub default_lease_time: i32,
    pub max_lease_time: i32,
    pub min_lease_time: i32,
}

#[derive(
    Identifiable, Serialize, Deserialize, Queryable, PartialEq, Clone, Debug, GraphQLObject,
)]
#[diesel(table_name = super::db::server)]
#[diesel(primary_key(id))]
pub struct Server {
    #[diesel(sql_type = Integer)]
    pub id: i32,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub tokenhash: Option<String>,
    #[diesel(sql_type = Nullable<Timestamptz>)]
    pub lastcheckin: Option<NaiveDateTime>,
    #[diesel(sql_type = Bool)]
    pub dns_update: bool,
    #[diesel(sql_type = Bool)]
    pub dhcp_update: bool,
}

#[derive(
    Identifiable, Serialize, Deserialize, Queryable, PartialEq, Clone, Debug, GraphQLObject,
)]
#[diesel(table_name = super::db::ddns)]
#[diesel(primary_key(iprangeid, zoneid))]
pub struct DDNS {
    pub iprangeid: i32,
    pub zoneid: i32,
}

#[derive(
    Identifiable, Serialize, Deserialize, Queryable, PartialEq, Clone, Debug, GraphQLObject,
)]
#[diesel(table_name = super::db::dnszone)]
#[diesel(primary_key(id))]
pub struct DNSZone {
    pub id: i32,
    pub zonename: String,
    pub serverid: i32,
}

#[derive(juniper::GraphQLEnum, Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum DNSRecordType {
    A,
    AAAA,
}

#[derive(
    Identifiable, Serialize, Deserialize, Queryable, PartialEq, Clone, Debug, GraphQLObject,
)]
#[diesel(table_name = super::db::dnsrecord)]
#[diesel(primary_key(id))]
pub struct DNSRecord {
    pub id: i32,
    pub zoneid: i32,
    pub key: String,
    pub recordtype: DNSRecordType,
    pub ttl: i32,
    pub value: String,
}

#[derive(
    Identifiable, Serialize, Deserialize, Queryable, PartialEq, Clone, Debug, GraphQLObject,
)]
#[diesel(table_name = super::db::dhcprange)]
#[diesel(primary_key(id))]
pub struct DHCPRange {
    pub id: i32,
    pub iprangeid: i32,
    pub name: String,
    pub dhcpstart: String,
    pub dhcpend: String,
    pub gateway: String,
    pub default_dns: String,
    pub lease_time: i32,
    pub serverid: i32,
}
