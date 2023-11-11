// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "dnsrecordtype"))]
    pub struct Dnsrecordtype;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "iptype"))]
    pub struct Iptype;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ipversion"))]
    pub struct Ipversion;
}

diesel::table! {
    _sqlx_migrations (version) {
        version -> Int8,
        description -> Text,
        installed_on -> Timestamptz,
        success -> Bool,
        checksum -> Bytea,
        execution_time -> Int8,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Iptype;

    address (id) {
        id -> Int4,
        interfaceid -> Int4,
        iprangeid -> Int4,
        iptype -> Nullable<Iptype>,
    }
}

diesel::table! {
    apikey (id) {
        id -> Int4,
        #[max_length = 64]
        name -> Varchar,
        #[max_length = 256]
        keyhash -> Bpchar,
    }
}

diesel::table! {
    ddns (iprangeid, zoneid) {
        iprangeid -> Int4,
        zoneid -> Int4,
    }
}

diesel::table! {
    device (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        owner -> Varchar,
        #[max_length = 255]
        comments -> Varchar,
        #[max_length = 50]
        group -> Varchar,
        group_edit -> Bool,
    }
}

diesel::table! {
    dhcprange (id) {
        id -> Int4,
        iprangeid -> Int4,
        name -> Varchar,
        #[max_length = 255]
        dhcpstart -> Varchar,
        #[max_length = 255]
        dhcpend -> Varchar,
        jail -> Bool,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Dnsrecordtype;

    dnsrecord (id) {
        id -> Int4,
        zoneid -> Int4,
        #[max_length = 255]
        key -> Varchar,
        ttl -> Int4,
        #[max_length = 255]
        value -> Varchar,
        recordtype -> Dnsrecordtype,
        addrid -> Int4,
    }
}

diesel::table! {
    dnszone (id) {
        id -> Int4,
        #[max_length = 255]
        zonename -> Varchar,
        serverid -> Int4,
        #[max_length = 255]
        dnsroot -> Varchar,
        refresh -> Int4,
        retry -> Int4,
        expire -> Int4,
        nxdomain -> Int4,
        #[max_length = 255]
        contact -> Varchar,
        #[max_length = 255]
        soa -> Varchar,
    }
}

diesel::table! {
    group (name) {
        #[max_length = 50]
        name -> Varchar,
        #[max_length = 50]
        binding -> Varchar,
    }
}

diesel::table! {
    interface (id) {
        id -> Int4,
        #[max_length = 17]
        macaddr -> Bpchar,
        deviceid -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        comments -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Ipversion;

    iprange (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        ipversion -> Ipversion,
        #[max_length = 15]
        networkid -> Varchar,
        cidr -> Int4,
        #[max_length = 255]
        description -> Varchar,
        vlan -> Int4,
        #[max_length = 50]
        gateway -> Varchar,
        #[max_length = 50]
        default_dns -> Varchar,
        #[max_length = 50]
        dns_domain -> Varchar,
        default_lease_time -> Int4,
        max_lease_time -> Int4,
        min_lease_time -> Int4,
    }
}

diesel::table! {
    keypermissions (id) {
        id -> Int4,
        keyid -> Int4,
        permission -> Varchar,
    }
}

diesel::table! {
    logs (id) {
        id -> Int4,
        timestamp -> Timestamptz,
        #[max_length = 255]
        message -> Nullable<Varchar>,
    }
}

diesel::table! {
    mxrecord (id) {
        id -> Int4,
        preference -> Int4,
    }
}

diesel::table! {
    server (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 256]
        tokenhash -> Nullable<Varchar>,
        lastcheckin -> Nullable<Timestamptz>,
        dnsupdate -> Bool,
        dhcpupdate -> Bool,
    }
}

diesel::table! {
    srvrecord (id) {
        id -> Int4,
        preference -> Int4,
        weight -> Int4,
        port -> Int4,
    }
}

diesel::table! {
    staticaddress (addressid) {
        addressid -> Int4,
        #[max_length = 15]
        ipaddr -> Varchar,
    }
}

diesel::table! {
    vlan (id) {
        id -> Int4,
        #[max_length = 50]
        name -> Varchar,
        serverid -> Int4,
    }
}

diesel::joinable!(address -> interface (interfaceid));
diesel::joinable!(address -> iprange (iprangeid));
diesel::joinable!(ddns -> dnszone (zoneid));
diesel::joinable!(ddns -> iprange (iprangeid));
diesel::joinable!(device -> group (group));
diesel::joinable!(dhcprange -> iprange (iprangeid));
diesel::joinable!(dnsrecord -> address (addrid));
diesel::joinable!(dnsrecord -> dnszone (zoneid));
diesel::joinable!(dnszone -> server (serverid));
diesel::joinable!(interface -> device (deviceid));
diesel::joinable!(iprange -> vlan (vlan));
diesel::joinable!(keypermissions -> apikey (keyid));
diesel::joinable!(mxrecord -> dnsrecord (id));
diesel::joinable!(srvrecord -> dnsrecord (id));
diesel::joinable!(staticaddress -> address (addressid));
diesel::joinable!(vlan -> server (serverid));

diesel::allow_tables_to_appear_in_same_query!(
    _sqlx_migrations,
    address,
    apikey,
    ddns,
    device,
    dhcprange,
    dnsrecord,
    dnszone,
    group,
    interface,
    iprange,
    keypermissions,
    logs,
    mxrecord,
    server,
    srvrecord,
    staticaddress,
    vlan,
);
