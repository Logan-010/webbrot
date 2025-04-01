use leptos::{prelude::*, task};
use webbrot::{encode_image, options::Options};

fn main() {
    tracing_wasm::set_as_global_default();
    task::Executor::init_wasm_bindgen().unwrap();

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let options = RwSignal::new(Options::default());
    let generate_image = Action::new(|o: &Options| encode_image(o.clone()));
    let image = generate_image.value();

    view! {
        <div class="w-screen h-screen flex items-center justify-center relative">
            <Transition fallback=|| {
                view! { <p class="text-white text-sm">Loading...</p> }
            }>

                {move || {
                    image
                        .get()
                        .map(|d| {
                            view! {
                                <img
                                    class="border-gray-800"
                                    src=move || format!("data:image/png;base64, {}", d)
                                    alt="Mandelbrot fractal image"
                                />
                            }
                        })
                }}

            </Transition>

            <div class="flex flex-col items-center justify-center text-white rounded-lg p-4 space-y-2 border-gray-800 border">
                <p class="text-white text-md">Mandelbrot fractal generator</p>

                <div class="items-center justify-center">
                    <p class="text-white text-sm">Width:</p>
                    <input
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value="960"
                        min="0"
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
                    <p class="text-white text-sm">Height:</p>
                    <input
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value="540"
                        min="0"
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
                    <p class="text-white text-sm">Center X:</p>
                    <input
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value=""
                        min="0"
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
                    <p class="text-white text-sm">Center Y:</p>
                    <input
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value=""
                        min="0"
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
                    <p class="text-white text-sm">View size X:</p>
                    <input
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value=""
                        min="0"
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
                    <p class="text-white text-sm">View size Y:</p>
                    <input
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value=""
                        min="0"
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
                    <p class="text-white text-sm">Steps min:</p>
                    <input
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
                    <p class="text-white text-sm">Steps max:</p>
                    <input
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value="1024"
                        min="0"
                        on:input=move |ev| {
                            options
                                .update(|o| {
                                    o
                                        .step_limits[0] = event_target_value(&ev)
                                        .parse::<f64>()
                                        .unwrap_or(webbrot::MAX_STEPS as f64) as u32;
                                })
                        }
                    />

                </div>

                <div class="items-center justify-center">
                    <p class="text-white text-sm">RNG Seed</p>
                    <input
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
                    <p class="text-white text-sm">Bailout number</p>
                    <input
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value="15"
                        min="0"
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

                <div class="items-center justify-center">
                    <p class="text-white text-sm">Chunk size</p>
                    <input
                        class="text-gray-500 text-sm border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white"
                        type="number"
                        value="15"
                        min="1"
                        on:input=move |ev| {
                            options
                                .update(|o| {
                                    o
                                        .chunk_size = event_target_value(&ev)
                                        .parse::<f64>()
                                        .unwrap_or(15.0) as usize;
                                })
                        }
                    />

                </div>

                <div class="items-center justify-center">
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
                            None
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

                <button
                    on:click=move |_| {generate_image.dispatch(options.get());}
                    class="border-gray-800 border-gray-800 bg-gray-950 hover:bg-gray-900 hover:text-white focus-visible:ring-gray-300 hover:underline inline-flex items-center rounded-md border border-gray-200 px-4 py-2 text-sm font-medium shadow-sm transition-colors hover:bg-gray-100 hover:text-gray-900 focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-gray-950"
                >
                    Generate
                </button>
            </div>
        </div>
    }
}
