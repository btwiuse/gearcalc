#![no_std]

use codec::Ops;
use gstd::ActorId;
use gstd::ToString;

async fn add(left: i128, right: i128) -> i128 {
    let x = gstd::msg::send_for_reply_as::<_, Ops>(
        ActorId::new(hex_literal::hex!(
            "b982692d879d5d007fe85990944dcee6003e972d26ba5d29b8b2b0e886606731"
        )),
        Ops::Add(left, right),
        0,
    )
    .unwrap()
    .await
    .unwrap();
    match x {
        Ops::Int128(i) => i,
        _ => 0,
    }
}

async fn sub(left: i128, right: i128) -> i128 {
    let x = gstd::msg::send_for_reply_as::<_, Ops>(
        ActorId::new(hex_literal::hex!(
            "2628ef3dc3e9fc20841b6dbafc7aa0d07ee6a8c280408ecafec4ad31942c1d2d"
        )),
        Ops::Sub(left, right),
        0,
    )
    .unwrap()
    .await
    .unwrap();
    match x {
        Ops::Int128(i) => i,
        _ => 0,
    }
}

async fn div(left: i128, right: i128) -> i128 {
    let x = gstd::msg::send_for_reply_as::<_, Ops>(
        ActorId::new(hex_literal::hex!(
            "f6d81379de198461983d858c22e7e5df7f94ac92da7543619e254360ffd982ec"
        )),
        Ops::Div(left, right),
        0,
    )
    .unwrap()
    .await
    .unwrap();
    match x {
        Ops::Int128(i) => i,
        _ => 0,
    }
}

async fn mult(left: i128, right: i128) -> i128 {
    let x = gstd::msg::send_for_reply_as::<_, Ops>(
        ActorId::new(hex_literal::hex!(
            "c6b01960c37fd23d32ad5a2aa438b0fd5c50d1235cd2a91953ff9a6bef93b2df"
        )),
        Ops::Mult(left, right),
        0,
    )
    .unwrap()
    .await
    .unwrap();
    match x {
        Ops::Int128(i) => i,
        _ => 0,
    }
}

async fn pow(left: i128, right: u32) -> i128 {
    let x = gstd::msg::send_for_reply_as::<_, Ops>(
        ActorId::new(hex_literal::hex!(
            "2c8a8f52ea72325685eb429799f5a69e71f904a955efdd0e5f46dee352f1a0e3"
        )),
        Ops::Pow(left, right),
        0,
    )
    .unwrap()
    .await
    .unwrap();
    match x {
        Ops::Int128(i) => i,
        _ => 0,
    }
}

async fn modulus(left: i128, right: i128) -> i128 {
    let x = gstd::msg::send_for_reply_as::<_, Ops>(
        ActorId::new(hex_literal::hex!(
            "c27dd9384d11beafc4e99dfbbb4b2b8dc976895a47b2b02e11c0c6e8db7664bf"
        )),
        Ops::Modulus(left, right),
        0,
    )
    .unwrap()
    .await
    .unwrap();
    match x {
        Ops::Int128(i) => i,
        _ => 0,
    }
}

async fn neg(left: i128) -> i128 {
    let x = gstd::msg::send_for_reply_as::<_, Ops>(
        ActorId::new(hex_literal::hex!(
            "86ad9dd2c43e062ea50b366b7d80cb663df9d513f5c634fc05fc24fc314aed1a"
        )),
        Ops::Neg(left),
        0,
    )
    .unwrap()
    .await
    .unwrap();
    match x {
        Ops::Int128(i) => i,
        _ => 0,
    }
}

#[gstd::async_main]
async fn main() {
    let ops: Ops = gstd::msg::load().expect("failed to load action");
    match ops {
        Ops::Add(left, right) => {
            gstd::msg::reply(Ops::Int128(add(left, right).await), 0).expect("failed to reply");
        }
        Ops::Sub(left, right) => {
            gstd::msg::reply(Ops::Int128(sub(left, right).await), 0).expect("failed to reply");
        }
        Ops::Mult(left, right) => {
            gstd::msg::reply(Ops::Int128(mult(left, right).await), 0).expect("failed to reply");
        }
        Ops::Div(left, right) => {
            gstd::msg::reply(Ops::Int128(div(left, right).await), 0).expect("failed to reply");
        }
        Ops::Modulus(left, right) => {
            gstd::msg::reply(Ops::Int128(modulus(left, right).await), 0).expect("failed to reply");
        }
        Ops::Pow(left, right) => {
            gstd::msg::reply(Ops::Int128(pow(left, right).await), 0).expect("failed to reply");
        }
        Ops::Neg(left) => {
            gstd::msg::reply(Ops::Int128(neg(left).await), 0).expect("failed to reply");
        }
        Ops::Int128(left) => {
            gstd::msg::reply(Ops::Int128(left), 0).expect("failed to reply");
        }
    }
}

gstd::metadata! {
    title: "gearcalc",
    handle:
        input: Ops,
        output: Ops,
}
