#![allow(non_snake_case)]
use dioxus::prelude::*;
// use reqwest::{self};
use wasm_bindgen::prelude::*;

use rand::prelude::*;
use serde::Deserialize;
// use serde_json::json;

// #[tokio::main]
fn main() {
  // Init logger
  // dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
  dioxus_web::launch(App);
}

// #[derive(Deserialize, Debug)]
// struct ApiResponse {
//   message: String,
//   status: String,
// }

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

// #[tokio::main]
fn App(cx: Scope) -> Element {
  let my_hand  = use_state(cx, || 0);
  let cpu_hand = use_state(cx, || 0);
  let result = use_state(cx, || "");
  // let resp = use_state(cx, || ApiResponse {
  //   message: "".to_string(),
  //   status: "".to_string(),
  // });

  // let get_img_url = move |_| {
  //   cx.spawn({
  //     let resp = resp.to_owned();      
  //     async move {
  //       let res = reqwest::get("https://dog.ceo/api/breeds/image/random")
  //         // .send()
  //         .await
  //         .unwrap()
  //         .json::<ApiResponse>()
  //         .await;
    
  //       log(&format!("response: {:?}", res));
        
  //       match res {
  //         Ok(data) => {
  //           resp.set(data);
  //         },
  //         Err(_) => {
  //           log("error");
  //         }
  //       }

  //     }


  //   })
  // };

  let hands = [
    "https://jskm.sakura.ne.jp/js01/kadai/img02/g.png",
    "https://jskm.sakura.ne.jp/js01/kadai/img02/c.png",
    "https://jskm.sakura.ne.jp/js01/kadai/img02/p.png"
  ];

  cx.render(rsx! {
    // div {
    //   resp.get().message
    // }
    // button {
    //   onclick: get_img_url,
    //   "get image"
    // }
    div {
      h3 {
        style: "text-align: center;",
        "じゃんけん"
      }
      p {
        style: "text-align: center;",
        "あなたの手を選んでください"
      }
      div {
        style: "display: flex; gap: 12px; align-items: center; justify-content: center;",
        hands.iter().enumerate().map(|(i, &hand)| cx.render(rsx!{
          button {
            style: "padding: 4px 8px; background-color: #000; border-radius: 12px;",
            onclick: move |_| { 
              my_hand.set(i + 1);
              let tmp_hand = dicide_cpu_hand();
              cpu_hand.set(tmp_hand);
              result.set(janken(i + 1, tmp_hand ));
            },
            img {
              width: "64px",
              src: hand,
            },
          }
        }))
      }
      div  {
        style: "display: flex; gap: 24px; align-items: center; justify-content: center; margin-top: 64px",
        div {
          style: "display: flex; flex-direction: column; align-items: center; justify-content: center;",
          div {
            style: "padding: 4px 8px; background-color: blue; border-radius: 12px; width: 64px;height: 64px;",
            img {
              width: "64px",
              src: if my_hand.get().clone() != 0 {hands[my_hand.get() - 1 as usize]} else {""},
            }
          }
          p {
            style: "height: 12px; margin: 0;",
            "YOU"
          }
        }

        div {
          style: "display: flex; flex-direction: column; align-items: center; justify-content: center;",
          div {
            style: "padding: 4px 8px; background-color: red; border-radius: 12px; width: 64px;height: 64px;",
            img {
              width: "64px",
              src: if my_hand.get().clone() != 0 { hands[cpu_hand.get() - 1 as usize] } else {""},
            },
          }
          p {
            style: "height: 12px; margin: 0;",
            "CPU"
          }
        }

      }

      div {
        style: "margin-top:64px;",
        h3 {
          style: "text-align: center;",
          result.get() as &str
        }
      }
      if result.get().clone() == "あなたの負け" {
        rsx! {
          div {
            style: "margin: 0 auto; width: 120px;",
            button {
              "Chat GPTに聞く"
            }
  
          }
        }
      }
    }
  })
}


fn dicide_cpu_hand() -> usize {
  let mut rng = rand::thread_rng();
  rng.gen_range(1..4)
}

fn janken(my_hand: usize, cpu_hand: usize) -> &'static str {
  let result = (my_hand + 3 - cpu_hand) % 3;
  let result = match result {
    0 => "あいこ",
    1 => "あなたの負け",
    2 => "あなたの勝ち",
    _ => "エラー",
  };
  log(&format!("{} {} {}", my_hand, cpu_hand, result));
  result
}

// #[tokio::main]
// async fn chat () {
//   let body = reqwest::get("https://www.rust-lang.org")
//     .await
//     .unwrap()
//     .text()
//     .await
//     .unwrap();
//   log(&format!("response: {}", body));
// }