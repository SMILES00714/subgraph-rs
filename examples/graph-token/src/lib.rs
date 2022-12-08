use subgraph::{
    address::Address,
    conv, crypto, datasource, ens, entity, eth, json, log,
    num::{BigDecimal, BigInt},
    store,
};

#[no_mangle]
pub extern "C" fn call_me() {
    log::log(log::Level::Debug, "called me");

    let answer = BigInt::new(42);
    log::log(
        log::Level::Info,
        &format!("The Answer to Life, The Universe, and Everything is {answer}!"),
    );

    for x in [BigInt::new(42), BigInt::new(-42)] {
        for s in [
            format!("{x:?}"),
            format!("{x:x?}"),
            format!("{x:+X?}"),
            format!("{x:#?}"),
            format!("{x:#x?}"),
            format!("{x:+#X?}"),
            format!("{x}"),
            format!("{x:+}"),
            format!("{x:x}"),
            format!("{x:+X}"),
            format!("{x:#x}"),
            format!("{x:+#X}"),
        ] {
            log::log(log::Level::Info, &s);
        }
    }

    let json = json::Value::from_bytes(
        r#"
            [
                {
                    "hello": "world",
                    "missing": null,
                    "isCool": true
                },
                42.1
            ]
        "#,
    );
    log::log(log::Level::Info, &format!("{json:?} => {json}"));

    for json in [
        r#""#,
        r#"invalid"#,
        r#"null"#,
        r#"{"field":invalid}"#,
        r#"{"field":null}"#,
        r#"{"field":null,}"#,
    ] {
        let json = json::Value::try_from_bytes(json);
        log::log(log::Level::Info, &format!("{json:?}"));
    }

    let json = json::Value::from_bytes(
        r#"
            {
                "big": 115792089237316195423570985008687907853269984665640564039457584007913129639935,
                "float": 13.37,
                "signed": -42,
                "unsigned": 42
            }
        "#,
    );
    let jnum = |name: &str| json.as_object().unwrap()[name].as_number().unwrap();
    log::log(log::Level::Info, &format!("{}", jnum("big").to_big_int()));
    log::log(log::Level::Info, &format!("{}", jnum("float").to_f64()));
    log::log(log::Level::Info, &format!("{}", jnum("signed").to_i64()));
    log::log(log::Level::Info, &format!("{}", jnum("unsigned").to_u64()));

    let digest = crypto::keccak256("Hello Subgraph");
    log::log(log::Level::Info, &format!("{digest:x?}"));

    let address = Address::parse("0xDEf1CA1fb7FBcDC777520aa7f396b4E015F497aB");
    log::log(log::Level::Info, &format!("{address}"));

    let dec = BigDecimal::new(42);
    log::log(log::Level::Info, &format!("{dec} <> {dec:?}"));

    let bytes = b"hello";
    for value in [
        conv::hex(bytes),
        conv::base58(bytes),
        #[allow(deprecated)]
        conv::string(bytes),
    ] {
        log::log(log::Level::Info, &value);
    }

    log::log(
        log::Level::Info,
        &format!(
            "data source: network {}, address {}, context {:?}",
            datasource::network(),
            datasource::address(),
            datasource::context(),
        ),
    );

    // The test-bench doesn't seem to like templated data sources. Uncommenting
    // any of these lines will cause a panic, but the data source creation with
    // the provided parameters, which is good enough for me:
    // ```
    // INFO Create data source, params: foo,bar, name: example template
    // ```
    //datasource::create("example template", ["foo", "bar"]);
    //datasource::create_with_context(
    //    "example template",
    //    ["foo", "bar"],
    //    &subgraph::indexmap::indexmap! {
    //        "foo".to_owned() => entity::Value::String("bar".to_owned()),
    //        "number".to_owned() => entity::Value::Int(42),
    //        "dec".to_owned() =>
    //            entity::Value::BigDecimal(BigDecimal::new(42)),
    //        "isGood".to_owned() => entity::Value::Bool(true),
    //        "many".to_owned() => entity::Value::Array(vec![
    //            entity::Value::Null,
    //            entity::Value::Bytes(vec![1, 2, 3]),
    //            entity::Value::BigInt(BigInt::new(-1)),
    //        ]),
    //    },
    //);

    let check_storage = || {
        let data = store::get("Thing", "0");
        log::log(log::Level::Info, &format!("{data:?}"));
    };

    check_storage();
    store::set(
        "Thing",
        "0",
        &subgraph::indexmap::indexmap! {
            "foo".to_owned() => entity::Value::String("bar".to_owned()),
            "number".to_owned() => entity::Value::Int(42),
            "dec".to_owned() =>
                entity::Value::BigDecimal(BigDecimal::new(42)),
            "isGood".to_owned() => entity::Value::Bool(true),
            "many".to_owned() => entity::Value::Array(vec![
                entity::Value::Null,
                entity::Value::Bytes(vec![1, 2, 3]),
                entity::Value::BigInt(BigInt::new(-1)),
            ]),
        },
    );
    check_storage();
    store::remove("Thing", "0");
    check_storage();

    log::log(
        log::Level::Info,
        &format!("{:?}", ens::name_by_hash("echo")),
    );

    // The test-bench doesn't seem like IPFS calls, but it seems they are being
    // done though...
    //log::log(
    //    log::Level::Info,
    //    &format!(
    //        "{:?}",
    //        subgraph::ipfs::cat("QmNLei78zWmzUdbeRB3CiUfAizWUrbeeZh5K1rhAQKCh51"),
    //    ),
    //);
    //subgraph::ipfs::map(
    //    "QmNLei78zWmzUdbeRB3CiUfAizWUrbeeZh5K1rhAQKCh51",
    //    "callback_name",
    //    entity::Value::String("value".to_string()),
    //    &["foo", "bar"],
    //)

    let value = eth::Value::Tuple(vec![
        eth::Value::Address(Address([0xee; 20])),
        eth::Value::FixedBytes(vec![1, 2, 3, 4]),
        eth::Value::Bytes(vec![1, 2, 3, 4]),
        eth::Value::Int(BigInt::new(-42)),
        eth::Value::Uint(BigInt::new(42)),
        eth::Value::Bool(true),
        eth::Value::String("My tuple".to_string()),
        eth::Value::FixedArray(vec![
            eth::Value::Uint(BigInt::new(1)),
            eth::Value::Uint(BigInt::new(2)),
            eth::Value::Uint(BigInt::new(3)),
        ]),
        eth::Value::Array(vec![
            eth::Value::Uint(BigInt::new(1)),
            eth::Value::Uint(BigInt::new(2)),
            eth::Value::Uint(BigInt::new(3)),
        ]),
        eth::Value::Tuple(vec![
            eth::Value::Uint(BigInt::new(1)),
            eth::Value::Uint(BigInt::new(2)),
            eth::Value::Uint(BigInt::new(3)),
        ]),
    ]);
    log::log(log::Level::Info, &format!("{value:?}"));
    let encoded = eth::encode(&value).unwrap();
    log::log(log::Level::Info, &conv::hex(&encoded));
    let decoded = eth::decode(
        "(address,bytes4,bytes,int256,uint256,bool,string,uint[3],uint[],(uint,uint,uint))",
        &encoded,
    )
    .unwrap();
    log::log(log::Level::Info, &format!("{decoded:?}"));
    log::log(
        log::Level::Info,
        &format!("encoded == decoded ? {}", value == decoded),
    );

    let result = eth::call(eth::SmartContractCall {
        contract: eth::Contract {
            name: "CowToken",
            address: &Address::parse("0xDEf1CA1fb7FBcDC777520aa7f396b4E015F497aB"),
        },
        function: eth::Function {
            name: "balanceOf",
            signature: "balanceOf(uint256):(uint256)",
        },
        params: &[eth::Value::Address(Address([0xee; 20]))],
    })
    .unwrap();
    log::log(log::Level::Info, &format!("{result:?}"));

    // Doesn't work as expected...
    // FIXME(nlordell): Figure out why this is allowed...
    //let invalid = eth::encode(&eth::Value::FixedBytes(vec![0; 33]));
    //let invalid = eth::encode(&eth::Value::FixedBytes(vec![0;33]));
    //    eth::Value::Bool(true),
    //    eth::Value::Int(BigInt::new(-42)),
    //]));
    //let invalid = eth::encode(&eth::Value::Uint(BigInt::from_signed_bytes_le(&[1; 33])));
    //log::log(log::Level::Info, &format!("{invalid:?}"));

    let hundred = BigInt::new(100);
    for value in [
        BigInt::new(1).plus(&BigInt::new(1)),
        BigInt::parse("124").minus(&BigInt::new(1)),
        BigInt::new(10).times(&BigInt::new(10)),
        BigInt::new(420).divided_by(&BigInt::new(10)),
        BigInt::new(15).rem(&BigInt::new(11)),
        BigInt::new(2).pow(8),
        BigInt::new(0b1001).bit_or(&BigInt::new(0b1010)),
        BigInt::new(0b1001).bit_and(&BigInt::new(0b1010)),
        BigInt::new(1).left_shift(8),
        BigInt::new(-2).right_shift(1),
    ] {
        log::log(
            log::Level::Info,
            &format!(
                "bigint {} {:?} 100 (= {})",
                value,
                value.cmp(&hundred),
                value == hundred
            ),
        );
    }

    let one = BigDecimal::new(1);
    for value in [
        BigDecimal::parse("0.337").plus(&BigDecimal::parse("1")),
        BigDecimal::parse("1.337").minus(&BigDecimal::parse("1")),
        BigDecimal::parse("2").times(&BigDecimal::parse("0.5")),
        BigDecimal::parse("42").divided_by(&BigDecimal::parse("12")),
        BigInt::new(3).divided_by_decimal(&BigDecimal::parse("2.5")),
    ] {
        log::log(
            log::Level::Info,
            &format!(
                "bigdecimal {} {:?} 1 (= {})",
                value,
                value.cmp(&one),
                value == one
            ),
        );
    }
}
