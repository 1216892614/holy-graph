mod ast;
mod get_node;
mod prime_match;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let dices = use_state(|| Vec::new());
    let level = use_state(|| 1usize);

    let get_dices = {
        let dices = dices.clone();
        Callback::from(move |e: Event| {
            let input = e.target().unwrap().unchecked_into::<HtmlInputElement>();
            let mut arr = Vec::new();
            for d in input.value().chars() {
                let d = match d.to_string().parse::<f32>() {
                    Ok(f) => f,
                    Err(_) => continue,
                };
                arr.push(d);
            }
            dices.set(arr);
        })
    };

    let get_level = {
        let level = level.clone();
        Callback::from(move |e: Event| {
            let lv = match e
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse::<usize>()
            {
                Ok(v) => v,
                Err(_) => return,
            };
            level.set(lv);
        })
    };

    let formula = use_state(|| None);

    let get_one_case = {
        let dices = dices.clone();
        let level = level.clone();
        let formula = formula.clone();
        Callback::from(move |_| {
            let ast = get_node::dfs_one(&*dices, *level);
            formula.set(ast)
        })
    };

    let get_all_case = {
        let formula = formula.clone();
        Callback::from(move |_| {
            let ast = get_node::dfs_all(&*dices, *level);
            formula.set(ast)
        })
    };

    html! {
        <>
        <h1>{ "圣神鸡盒" }</h1>
        <h2>{ "骰子:" }</h2>
        <input
            onchange={get_dices}
            placeholder="直接114514一样敲数字"
        />
        <h2>{ "等级:" }</h2>
        <input
            onchange={get_level}
            placeholder="Rua"
        />
        <hr/>
        <div>
            <button onclick={get_one_case}>{"进行一个计的算"}</button>
            <button onclick={get_all_case}>{"进行全部计的算"}</button>
        </div>
        <h2>{ "结果:" }</h2>
        <div>
        {
            if let Some(arr) = &*formula {
                arr.iter().map(|i| {
                    html!{
                        <>
                            <a style="color: green; border: 1px;">{format!("{}",i)}</a><br/>
                        </>
                }
                }).collect::<Html>()
            } else {
                html!{<h1 style="color: blueviolet;">{"寄!"}</h1>}
            }
        }
        </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
