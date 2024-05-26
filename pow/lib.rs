#![no_std]

use async_recursion::async_recursion;
use codec::Ops;
use gstd::Box;

async fn mult(left: i128, right: i128) -> i128 {
    let x = gstd::msg::send_for_reply_as::<_, Ops>(
        gstd::actor_id!("0xc6b01960c37fd23d32ad5a2aa438b0fd5c50d1235cd2a91953ff9a6bef93b2df"),
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

#[async_recursion]
async fn pow(left: i128, right: u32) -> i128 {
    match right {
        0 => 1,
        1 => left,
        p => mult(left, pow(left, p - 1).await).await,
    }
}

#[gstd::async_main]
async fn main() {
    let ops: Ops = gstd::msg::load().expect("failed to load action");
    match ops {
        Ops::Pow(left, right) => {
            gstd::msg::reply(Ops::Int128(pow(left, right).await), 0).expect("failed to reply");
        }
        _ => (),
    }
}
