use leptos::*;
use webbrot::{encode_image, options::Options};

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let options = create_rw_signal(Options::default());
    let generate_image = create_action(|o: &Options| encode_image(o.clone()));
    let image = generate_image.value();

    view! {
        <Transition fallback=|| {
            view! { <p>Loading...</p> }
        }>

            {move || {
                image
                    .get()
                    .map(|d| {
                        view! {
                            <img
                                src=move || format!("data:image/png;base64, {}", d)
                                alt="Mandelbrot fractal image"
                            />
                        }
                    })
            }}

        </Transition>
        <br/>

        <p>Width:</p>
        <input
            type="number"
            value="960"
            min="0"
            on:input=move |ev| {
                options
                    .update(|o| {
                        o
                            .dimensions[0] = event_target_value(&ev).parse::<f64>().unwrap_or(960.0)
                            as u32;
                    })
            }
        />

        <br/>

        <p>Height:</p>
        <input
            type="number"
            value="540"
            min="0"
            on:input=move |ev| {
                options
                    .update(|o| {
                        o
                            .dimensions[1] = event_target_value(&ev).parse::<f64>().unwrap_or(540.0)
                            as u32;
                    })
            }
        />

        <br/>

        <p>Center X:</p>
        <input
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

        <br/>

        <p>Center Y:</p>
        <input
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

        <br/>

        <p>View size X:</p>
        <input
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

        <br/>

        <p>View size Y:</p>
        <input
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

        <br/>

        <p>Steps min:</p>
        <input
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

        <br/>

        <p>Steps max:</p>
        <input
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

        <br/>

        <p>RNG Seed</p>
        <input
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

        <br/>

        <p>Bailout number</p>
        <input
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

        <br/>

        <label for="colormap">Colormap:</label>
        <select
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
            <option value="none">None</option>
            {move || {
                webbrot::options::COLORMAP_CHOICES
                    .iter()
                    .map(|c| {
                        view! {
                            <option value=move || {
                                format!("{:?}", c)
                            }>{move || format!("{:?}", c)}</option>
                        }
                    })
                    .collect_view()
            }}
        </select>

        <br/>

        <button on:click=move |_| generate_image.dispatch(options.get())>Generate</button>
    }
}
