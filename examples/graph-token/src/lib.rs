use subgraph::{address::Address, entity, eth, indexmap::indexmap, log, num::BigInt, store};

/// `Transfer` event entry point.
///
/// # Safety
///
/// Should only ever be called by the Subgraph host.
#[no_mangle]
pub unsafe extern "C" fn transfer_handler(event: eth::EventPtr) {
    transfer(eth::Event::from_ptr(event));
}

fn transfer(event: eth::Event) {
    log::log(
        log::Level::Info,
        &format!("Transfer {:?}", event.parameters),
    );

    let value = event.parameters["value"].as_uint().unwrap();

    let from = holder(
        event.parameters["from"].as_address().unwrap(),
        BigInt::new(0).minus(value),
    );
    let to = holder(event.parameters["to"].as_address().unwrap(), value.clone());

    let id = event_id(&event);
    let data = indexmap! {
        "from".to_string()
            => from,
        "to".to_string()
            => to,
        "value".to_string()
            => entity::Value::BigInt(value.clone()),
        "blockNumber".to_string()
            => entity::Value::BigInt(event.block.number),
        "blockTimestamp".to_string()
            => entity::Value::BigInt(event.block.timestamp),
        "transactionHash".to_string()
            => entity::Value::Bytes(event.transaction.hash.to_vec()),
    };

    store::set("Transfer", &id, &data);
}

fn holder(address: Address, delta: BigInt) -> entity::Value {
    if address == Address::default() {
        return entity::Value::Null;
    }

    let id = address.to_string();
    let mut data = store::get("Holder", &id).unwrap_or_else(|| {
        indexmap! {
            "address".to_string()
                => entity::Value::Bytes(address.0.to_vec()),
            "balance".to_string()
                => entity::Value::BigInt(BigInt::new(0)),
        }
    });
    data["balance"] = entity::Value::BigInt(data["balance"].as_big_int().unwrap().plus(&delta));

    store::set("Holder", &id, &data);
    entity::Value::String(id)
}

fn event_id(event: &eth::Event) -> String {
    let bytes = eth::encode(&eth::Value::Tuple(vec![
        eth::Value::FixedBytes(event.transaction.hash.to_vec()),
        eth::Value::Uint(event.transaction_log_index.clone()),
    ]))
    .expect("failed to encode event ID");
    subgraph::conv::hex(&bytes)
}
