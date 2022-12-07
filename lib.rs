#![no_std]

use codec::Ops;
use gstd::ActorId;
use gstd::ToString;

async fn add(left: i128, right: i128) -> i128 {
    let x = gstd::msg::send_for_reply_as::<_, Ops>(
        ActorId::new(hex_literal::hex!(
            "2df243bbc717ccce2a462b82a255aa50187edc9c2ee6e9ca974eef8b842873ec"
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
            "2df243bbc717ccce2a462b82a255aa50187edc9c2ee6e9ca974eef8b842873ec"
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
            "2df243bbc717ccce2a462b82a255aa50187edc9c2ee6e9ca974eef8b842873ec"
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
            "ebb3ed0712ef2ba890ca6b8a317aba6be6c3598c127e9b9828c3c42dbb879815"
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
            "ebb3ed0712ef2ba890ca6b8a317aba6be6c3598c127e9b9828c3c42dbb879815"
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
            "ebb3ed0712ef2ba890ca6b8a317aba6be6c3598c127e9b9828c3c42dbb879815"
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
            "ebb3ed0712ef2ba890ca6b8a317aba6be6c3598c127e9b9828c3c42dbb879815"
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
