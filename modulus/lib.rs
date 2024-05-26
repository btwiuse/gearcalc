#![no_std]

use codec::Ops;
use gstd::ActorId;

async fn sub(left: i128, right: i128) -> i128 {
    let x = gstd::msg::send_for_reply_as::<_, Ops>(
        ActorId::new(hex_literal::hex!(
            "2628ef3dc3e9fc20841b6dbafc7aa0d07ee6a8c280408ecafec4ad31942c1d2d"
        )),
        Ops::Sub(left, right),
        0,
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
    sub(0, mult(div(left, right).await, right).await).await
}

#[gstd::async_main]
async fn main() {
    let ops: Ops = gstd::msg::load().expect("failed to load action");
    match ops {
        Ops::Modulus(n1, n2) => {
            gstd::msg::reply(Ops::Int128(modulus(n1, n2).await), 0).expect("failed to reply");
        }
        _ => (),
    }
}
