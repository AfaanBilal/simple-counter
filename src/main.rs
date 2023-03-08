/**
 * Simple Counter
 *
 * @author Afaan Bilal
 * @link   https://afaan.dev
 * @link   https://github.com/AfaanBilal/simple-counter
 * @link   https://afaan.dev/simple-counter
 */
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);

    let increment = {
        let counter = counter.clone();

        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let reset = {
        let counter = counter.clone();

        move |_| {
            counter.set(0);
        }
    };

    html! {
        <div class="h-screen w-full grid place-items-center">
            <div class="text-center">
                <div class="mb-6 pb-4 border-b border-b-slate-700">
                    <div class="text-slate-400 text-5xl mb-4">{ "A Simple Counter" }</div>
                    <div class="text-slate-400 text-xl">{ "Built with Rust, Yew and WASM" }</div>
                </div>
                <button class="w-60 h-60 text-slate-300 bg-slate-700 hover:bg-slate-600 active:bg-slate-500 border border-slate-800 font-mono text-extrabold rounded-full text-6xl " onclick={increment}>{ *counter }</button>
                <div class="my-8" />
                <button class="px-6 py-4 text-slate-300 bg-slate-700 hover:bg-slate-600 active:bg-slate-500 border border-slate-800 font-mono text-extrabold rounded text-4xl " onclick={reset}>{ "Reset" }</button>
                <div class="text-slate-500 mt-24">
                    { "\u{00A9} " }
                    <a class="text-cyan-400" href="https://afaan.dev" target="_blank" rel="noopener">{ "Afaan Bilal" }</a>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
