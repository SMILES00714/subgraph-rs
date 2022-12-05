use subgraph::{json, log, num::BigInt, crypto};

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

    let digest = crypto::keccak256("Hello Subgraph");
    log::log(log::Level::Info, &format!("{digest:x?}"));
}
