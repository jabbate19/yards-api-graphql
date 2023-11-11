use super::context::GraphQLContext;
use super::input::QueryServerInput;
use crate::db::db::address::{self, dsl::*};
use crate::db::db::device::{self, dsl::*};
use crate::db::db::interface::{self, dsl::*};
use crate::db::db::iprange::{self, dsl::*};
use crate::db::db::server::{self, dsl::*};
use crate::db::db::staticaddress::{self, dsl::*};
use crate::db::models::{Address, Device, IPRange, IPType, Interface, Server, StaticAddress};
use crate::schemas::input::QueryDeviceInput;
use diesel::prelude::*;
use juniper::{EmptyMutation, EmptySubscription, FieldResult, GraphQLObject, RootNode};

pub struct QueryRoot;

#[derive(GraphQLObject)]
struct AddressFull {
    id: i32,
    interfaceid: i32,
    iprangeid: i32,
    iptype: Option<IPType>,
    iprange: IPRange,
    staticaddress: Option<String>,
}

#[derive(GraphQLObject)]
struct InterfaceFull {
    id: i32,
    macaddr: String,
    deviceid: i32,
    name: String,
    comments: Option<String>,
    addresses: Vec<AddressFull>,
}

#[derive(GraphQLObject)]
struct DeviceFull {
    id: i32,
    name: String,
    owner: String,
    comments: String,
    group: String,
    group_edit: bool,
    interfaces: Vec<InterfaceFull>,
}

#[juniper::graphql_object(Context = GraphQLContext)]
impl QueryRoot {
    fn server(context: &GraphQLContext, input: QueryServerInput) -> FieldResult<Vec<Server>> {
        let mut query = server.into_boxed();
        if let Some(id_filter) = input.id {
            query = query.filter(server::columns::id.eq(id_filter));
        }
        if let Some(name_filter) = input.name {
            query = query.filter(server::columns::name.eq(name_filter));
        }
        if let Some(name_contains) = input.name_contains {
            query = query.filter(server::columns::name.like(format!("%{}%", name_contains)));
        }

        query
            .load::<Server>(&mut context.pool.get()?)
            .map_err(|e| e.into())
    }

    fn device(context: &GraphQLContext, input: QueryDeviceInput) -> FieldResult<Vec<DeviceFull>> {
        let mut query = device
            .inner_join(interface.inner_join(address.inner_join(iprange).left_join(staticaddress)))
            .group_by(device::id)
            .select(device::all_columns)
            .into_boxed();

        if let Some(id_filter) = input.id {
            query = query.filter(device::columns::id.eq(id_filter));
        }
        if let Some(name_filter) = input.name {
            query = query.filter(device::columns::name.eq(name_filter));
        }
        if let Some(name_contains) = input.name_contains {
            query = query.filter(device::columns::name.like(format!("%{}%", name_contains)));
        }
        if let Some(interface_has) = input.interface_has {
            if let Some(id_filter) = interface_has.id {
                query = query.filter(interface::columns::id.eq(id_filter));
            }
            if let Some(name_filter) = interface_has.name {
                query = query.filter(interface::columns::name.eq(name_filter));
            }
            if let Some(name_contains) = interface_has.name_contains {
                query = query.filter(interface::columns::name.like(format!("%{}%", name_contains)));
            }
            if let Some(address_has) = interface_has.address_has {
                if let Some(id_filter) = address_has.id {
                    query = query.filter(address::columns::id.eq(id_filter));
                }
                if let Some(static_address) = address_has.static_address {
                    
                }
                if let Some(static_address_subnet) = address_has.static_address_subnet {
                
                }
                if let Some(iprange_has) = address_has.iprange_has {

                }
            }
        }

        let devices = query.load::<Device>(&mut context.pool.get()?)?;

        let mut query = Interface::belonging_to(&devices).into_boxed();

        let interfaces = query.load::<Interface>(&mut context.pool.get()?)?;

        let mut query = Address::belonging_to(&interfaces)
            .inner_join(iprange)
            .left_join(staticaddress)
            .into_boxed();

        let addresses =
            query.load::<(Address, IPRange, Option<StaticAddress>)>(&mut context.pool.get()?)?;

        let out = addresses
            .grouped_by(&interfaces)
            .into_iter()
            .zip(interfaces)
            .map(|(addresses, int)| (int, addresses))
            .grouped_by(&devices)
            .into_iter()
            .zip(devices.clone())
            .map(|(interfaces, inner_device)| {
                let mut out_device = DeviceFull {
                    id: inner_device.id,
                    name: inner_device.name,
                    owner: inner_device.owner,
                    comments: inner_device.comments,
                    group: inner_device.group,
                    group_edit: inner_device.group_edit,
                    interfaces: Vec::new(),
                };
                out_device.interfaces = interfaces
                    .into_iter()
                    .map(|(inner_interface, addresses)| {
                        let mut out_interface = InterfaceFull {
                            id: inner_interface.id,
                            macaddr: inner_interface.macaddr,
                            deviceid: inner_interface.deviceid,
                            name: inner_interface.name,
                            comments: inner_interface.comments,
                            addresses: Vec::new(),
                        };
                        out_interface.addresses = addresses
                            .into_iter()
                            .map(|(inner_address, inner_iprange, inner_staticaddress)| {
                                AddressFull {
                                    id: inner_address.id,
                                    interfaceid: inner_address.interfaceid,
                                    iprangeid: inner_address.iprangeid,
                                    iptype: inner_address.iptype,
                                    iprange: inner_iprange,
                                    staticaddress: inner_staticaddress.map(|x| x.ipaddr),
                                }
                            })
                            .collect();
                        out_interface
                    })
                    .collect();
                out_device
            })
            .collect::<Vec<DeviceFull>>();

        Ok(out)
    }
}

pub type Schema =
    RootNode<'static, QueryRoot, EmptyMutation<GraphQLContext>, EmptySubscription<GraphQLContext>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
