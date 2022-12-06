use subgraph::{address::Address, conv, crypto, datasource, json, log, num::BigInt};

#[no_mangle]
pub extern "C" fn call_me() {
    log::log(log::Level::Debug, "called me");

    let answer = BigInt::temp_new(42);
    log::log(
        log::Level::Info,
        &format!("The Answer to Life, The Universe, and Everything is {answer}!"),
    );

    for x in [BigInt::temp_new(42), BigInt::temp_new(-42)] {
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

    let address = Address::new("0xDEf1CA1fb7FBcDC777520aa7f396b4E015F497aB");
    log::log(log::Level::Info, &format!("{address}"));

    let bytes = b"hello";
    for value in [
        conv::hex(bytes),
        conv::base58(bytes),
        #[allow(deprecated)]
        conv::string(bytes),
    ] {
        log::log(log::Level::Info, &value);
    }

    let datasource_address = datasource::address();
    log::log(log::Level::Info, &format!("{datasource_address}"));

    // The test-bench doesn't seem to like templated data sources. Uncommenting
    // this line will cause a panic, but the data source creation with the
    // provided parameters, which is good enough for me:
    // ```
    // INFO Create data source, params: foo,bar, name: example template
    // ```
    //datasource::create("example template", ["foo", "bar"]);
}
