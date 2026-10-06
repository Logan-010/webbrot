use wasm_bindgen::{closure::Closure, JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Blob, HtmlAnchorElement, HtmlCanvasElement, Url};

/// Encode the full drawing buffer asynchronously. Call immediately after a GPU redraw.
pub async fn save_png(canvas: HtmlCanvasElement) -> Result<(), String> {
    let filename = format!("webbrot-{}x{}.png", canvas.width(), canvas.height());
    let mut callback = None;
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let failed = reject.clone();
        callback = Some(Closure::once(move |blob: Option<Blob>| {
            if let Some(blob) = blob {
                let _ = resolve.call1(&JsValue::NULL, &blob);
            } else {
                let _ = failed.call1(
                    &JsValue::NULL,
                    &"The browser could not encode the PNG.".into(),
                );
            }
        }));
        if let Err(error) = canvas.to_blob(callback.as_ref().unwrap().as_ref().unchecked_ref()) {
            let _ = reject.call1(&JsValue::NULL, &error);
        }
    });
    let blob = JsFuture::from(promise)
        .await
        .map_err(|e| format!("PNG export failed: {e:?}"))?;
    drop(callback);
    let blob: Blob = blob
        .dyn_into()
        .map_err(|_| "The browser returned an invalid PNG.")?;
    let document = web_sys::window()
        .and_then(|w| w.document())
        .ok_or("Document is unavailable.")?;
    let link: HtmlAnchorElement = document
        .create_element("a")
        .map_err(|e| format!("Cannot create a download link: {e:?}"))?
        .dyn_into()
        .map_err(|_| "Cannot create a download link.")?;
    let url = Url::create_object_url_with_blob(&blob)
        .map_err(|e| format!("Cannot create a PNG URL: {e:?}"))?;
    link.set_href(&url);
    link.set_download(&filename);
    link.set_hidden(true);
    let body = document.body().ok_or("Document body is unavailable.")?;
    if let Err(error) = body.append_child(&link) {
        let _ = Url::revoke_object_url(&url);
        return Err(format!("Cannot attach the download link: {error:?}"));
    }
    link.click();
    link.remove();
    // Give the browser time to start consuming the URL before releasing its backing blob.
    let cleanup = Closure::once_into_js(move || {
        let _ = Url::revoke_object_url(&url);
    });
    web_sys::window()
        .unwrap()
        .set_timeout_with_callback_and_timeout_and_arguments_0(cleanup.unchecked_ref(), 10_000)
        .map_err(|e| format!("Cannot schedule download cleanup: {e:?}"))?;
    Ok(())
}
