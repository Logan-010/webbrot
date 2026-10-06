use leptos::{prelude::*, task};
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::JsCast;
use webbrot::{options::Options, renderer::Renderer};

fn main() {
    tracing_wasm::set_as_global_default();
    task::Executor::init_wasm_bindgen().unwrap();

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let options = RwSignal::new(Options::default());
    let canvas = NodeRef::<leptos::html::Canvas>::new();
    let renderer = Rc::new(RefCell::new(None::<Renderer>));
    let lost_renderer = Rc::clone(&renderer);
    let save_renderer = Rc::clone(&renderer);
    let status = RwSignal::new(String::new());
    let image_size = RwSignal::new(None::<[u32; 2]>);
    let saving = RwSignal::new(false);

    view! {
        <main class="app-shell">
            <header class="app-header">
                <h1>"WebBrot"</h1>
            </header>
            <div class="workspace">
            <section class="preview-panel" aria-label="Fractal preview">
                <div class="preview-toolbar">
                    <p class="muted resolution">
                        {move || image_size.get().map(|[w, h]| format!("{w} × {h} px")).unwrap_or_default()}
                    </p>
                    <button
                        id="save-image"
                        class="button button-secondary"
                        disabled=move || image_size.get().is_none() || saving.get()
                        on:click=move |_| {
                            saving.set(true);
                            status.set(String::new());
                            let renderer = Rc::clone(&save_renderer);
                            task::spawn_local(async move {
                                let result = async {
                                    let canvas = canvas.get().ok_or("Canvas is unavailable.")?;
                                    renderer.borrow().as_ref().ok_or("Generate an image before saving.")?.redraw()?;
                                    webbrot::download::save_png((*canvas).clone().unchecked_into()).await
                                }.await;
                                saving.set(false);
                                status.set(match result {
                                    Ok(()) => String::new(),
                                    Err(error) => error,
                                });
                            });
                        }
                    >
                        {move || if saving.get() { "Saving…" } else { "Save image" }}
                    </button>
                </div>
                <div class="preview-stage">
                <canvas
                    node_ref=canvas
                    width="960"
                    height="540"
                    class="fractal-canvas"
                    style:visibility=move || if image_size.get().is_some() { "visible" } else { "hidden" }
                    role="img"
                    aria-label="Mandelbrot fractal image"
                    on:webglcontextlost=move |ev: leptos::ev::Event| {
                        ev.prevent_default();
                        *lost_renderer.borrow_mut() = None;
                        image_size.set(None);
                        status.set("GPU context was lost.".into());
                    }
                    on:webglcontextrestored=move |_: leptos::ev::Event| {
                        status.set("GPU context has recovered. Generate again.".into());
                    }
                />
                </div>
                <p class="preview-status" role="status" aria-live="polite">
                    {move || status.get()}
                </p>
            </section>

            <aside class="controls-panel" aria-label="Render settings">
                <div class="fields-grid">

                <div class="items-center justify-center">
                    <label for="width">"Width (px)"</label>
                    <input
                        id="width"
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value="960"
                        min="2"
                        on:input=move |ev| {
                            options
                                .update(|o| {
                                    o
                                        .dimensions[0] = event_target_value(&ev)
                                        .parse::<f64>()
                                        .unwrap_or(960.0) as u32;
                                })
                        }
                    />

                </div>

                <div class="items-center justify-center">
                    <label for="height">"Height (px)"</label>
                    <input
                        id="height"
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value="540"
                        min="2"
                        on:input=move |ev| {
                            options
                                .update(|o| {
                                    o
                                        .dimensions[1] = event_target_value(&ev)
                                        .parse::<f64>()
                                        .unwrap_or(540.0) as u32;
                                })
                        }
                    />

                </div>

                <div class="items-center justify-center">
                    <label for="center-x">"Center X"</label>
                    <input
                        id="center-x"
                        placeholder="Random"
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value=""
                        step="any"
                        on:input=move |ev| {
                            options
                                .update(|o| {
                                    match event_target_value(&ev).parse::<f64>() {
                                        Ok(n) => {
                                            if let Some(ref mut c) = o.image_center {
                                                c[0] = n;
                                            } else {
                                                o.image_center = Some([n, 0.0]);
                                            }
                                        }
                                        Err(_) => o.image_center = None,
                                    }
                                })
                        }
                    />

                </div>

                <div class="items-center justify-center">
                    <label for="center-y">"Center Y"</label>
                    <input
                        id="center-y"
                        placeholder="Random"
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value=""
                        step="any"
                        on:input=move |ev| {
                            options
                                .update(|o| {
                                    match event_target_value(&ev).parse::<f64>() {
                                        Ok(n) => {
                                            if let Some(ref mut c) = o.image_center {
                                                c[1] = n;
                                            } else {
                                                o.image_center = Some([0.0, n]);
                                            }
                                        }
                                        Err(_) => o.image_center = None,
                                    }
                                })
                        }
                    />

                </div>

                <div class="items-center justify-center">
                    <label for="view-x">"View size X"</label>
                    <input
                        id="view-x"
                        placeholder="Auto"
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value=""
                        min="0"
                        step="any"
                        on:input=move |ev| {
                            options
                                .update(|o| {
                                    match event_target_value(&ev).parse::<f64>() {
                                        Ok(n) => {
                                            if let Some(ref mut c) = o.view_size {
                                                c[0] = n;
                                            } else {
                                                o.view_size = Some([n, 0.0]);
                                            }
                                        }
                                        Err(_) => o.view_size = None,
                                    }
                                })
                        }
                    />

                </div>

                <div class="items-center justify-center">
                    <label for="view-y">"View size Y"</label>
                    <input
                        id="view-y"
                        placeholder="Auto"
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value=""
                        min="0"
                        step="any"
                        on:input=move |ev| {
                            options
                                .update(|o| {
                                    match event_target_value(&ev).parse::<f64>() {
                                        Ok(n) => {
                                            if let Some(ref mut c) = o.view_size {
                                                c[1] = n;
                                            } else {
                                                o.view_size = Some([0.0, n]);
                                            }
                                        }
                                        Err(_) => o.view_size = None,
                                    }
                                })
                        }
                    />

                </div>

                <div class="items-center justify-center">
                    <label for="steps-min">"Minimum steps"</label>
                    <input
                        id="steps-min"
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value="150"
                        min="0"
                        on:input=move |ev| {
                            options
                                .update(|o| {
                                    o
                                        .step_limits[0] = event_target_value(&ev)
                                        .parse::<f64>()
                                        .unwrap_or(webbrot::MIN_STEPS as f64) as u32;
                                })
                        }
                    />

                </div>

                <div class="items-center justify-center">
                    <label for="steps-max">"Maximum steps"</label>
                    <input
                        id="steps-max"
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value="1024"
                        min="2"
                        on:input=move |ev| {
                            options
                                .update(|o| {
                                    o
                                        .step_limits[1] = event_target_value(&ev)
                                        .parse::<f64>()
                                        .unwrap_or(webbrot::MAX_STEPS as f64) as u32;
                                })
                        }
                    />

                </div>

                <div class="items-center justify-center">
                    <label for="seed">"Random seed"</label>
                    <input
                        id="seed"
                        placeholder="Auto"
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value=""
                        min="0"
                        on:input=move |ev| {
                            options
                                .update(|o| {
                                    match event_target_value(&ev).parse::<f64>() {
                                        Ok(n) => o.rng_seed = Some(n as u64),
                                        Err(_) => o.rng_seed = None,
                                    }
                                })
                        }
                    />

                </div>

                <div class="items-center justify-center">
                    <label for="bailout">"Bailout exponent"</label>
                    <input
                        id="bailout"
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value="15"
                        min="0"
                        max="38"
                        step="any"
                        on:input=move |ev| {
                            options
                                .update(|o| {
                                    o
                                        .bailout_num = event_target_value(&ev)
                                        .parse::<f64>()
                                        .unwrap_or(webbrot::BAILOUT_NUM);
                                })
                        }
                    />

                </div>

                <div class="palette-field">
                    <label class="text-white text-sm" for="colormap">
                        Colormap:
                    </label>
                    <select
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        name="colormap"
                        id="colormap"
                        on:change=move |ev| {
                            let selected_value = event_target_value(&ev);
                            options
                                .update(|o| {
                                    o
                                        .colormap = webbrot::options::COLORMAP_CHOICES
                                        .iter()
                                        .find(|c| format!("{:?}", c) == selected_value)
                                        .copied();
                                });
                        }
                    >

                        <option value="none" class="text-white text-sm">
                            Random palette
                        </option>
                        {move || {
                            webbrot::options::COLORMAP_CHOICES
                                .iter()
                                .map(|c| {
                                    view! {
                                        <option
                                            value=move || { format!("{:?}", c) }
                                            class="text-white text-sm"
                                        >
                                            {move || format!("{:?}", c)}
                                        </option>
                                    }
                                })
                                .collect_view()
                        }}

                    </select>
                </div>
                </div>
                <button
                    id="generate-image"
                    disabled=move || saving.get()
                    on:click=move |_| {
                        let result = (|| {
                            let mut renderer = renderer.borrow_mut();
                            if renderer.as_ref().is_some_and(Renderer::is_context_lost) {
                                *renderer = None;
                            }
                            if renderer.is_none() {
                                let canvas = canvas.get().ok_or("Canvas is not mounted yet.")?;
                                *renderer = Some(Renderer::new((*canvas).clone().unchecked_into())?);
                            }
                            let result = renderer.as_ref().unwrap().render(&options.get());
                            if renderer.as_ref().unwrap().has_image() {
                                let canvas = canvas.get().unwrap();
                                image_size.set(Some([canvas.width(), canvas.height()]));
                            } else {
                                image_size.set(None);
                            }
                            result
                        })();
                        status.set(match result {
                            Ok(()) => "Rendered".into(),
                            Err(error) => error,
                        });
                    }
                    class="button button-primary"
                >
                    Generate
                </button>
            </aside>
            </div>
        </main>
    }
}
