#![allow(non_snake_case)]
use dioxus::prelude::*;
use reqwest::{self, Error};
use wasm_bindgen::prelude::*;
use rand::prelude::*;
use serde::Deserialize;

fn main() {
  dioxus_web::launch(App);
}

#[derive(Deserialize, Debug, Clone)]
struct ApiResponse {
  message: String,
  status: String,
}

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

fn App(cx: Scope) -> Element {
  let my_hand  = use_state(cx, || 0);
  let cpu_hand = use_state(cx, || 0);
  let result = use_state(cx, || "");

  let image_url = use_state(cx, || ApiResponse{
    message: String::new(),
    status: String::new(), 
  });

  let get_image_url = move |_| {
    cx.spawn({
      let image_url = image_url.to_owned();

      async move {
        let res: Result<ApiResponse, Error> = 
          reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json()
            .await;
        
        match res {
          Ok(_data) => {
            log(&format!("status: {}", _data.status));
            image_url.set(_data);
          }
          Err(_err) => {
            log(&format!("Error get image url : {:?}", _err));
          }
        }
      }
    })
  };

  let hands = [
    "https://gs.mimoro.dev/js01/img02/g.png",
    "https://gs.mimoro.dev/js01/img02/c.png",
    "https://gs.mimoro.dev/js01/img02/p.png"
  ];

  cx.render(rsx! {
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
              let tmp_cpu_hand = dicide_cpu_hand();
              cpu_hand.set(tmp_cpu_hand);
              result.set(janken(i + 1, tmp_cpu_hand));
              image_url.set(ApiResponse{
                message: String::new(),
                status: String::new(),
              });
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
              src: if my_hand.get().clone() != 0 { hands[my_hand.get() - 1 as usize] } else {""},
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
      if *result.get() == "負け・・・" {
        rsx! {
          div {
            style: "margin: 0 auto; width: 144px; display:flex; flex-direction: column; align-items:center;",
            button {
              onclick: get_image_url,
              "傷ついた心を癒してもらうわん"
            }
            img {
              style: "margin-top: 24px; max-height: 480px",
              src: &image_url.get().message as &str
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
    1 => "負け・・・",
    2 => "勝ち！",
    _ => "エラー",
  };
  log(&format!("{} {} {}", my_hand, cpu_hand, result));
  result
}
