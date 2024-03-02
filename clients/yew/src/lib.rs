use restless_gloo::GlooRequest;
use std::{borrow::Cow, rc::Rc};
use yew::functional::hook;
use yew_hooks::prelude::{use_async_with_options, UseAsyncHandle, UseAsyncOptions};

pub type UseRequestHandle<R, E> = UseAsyncHandle<R, Rc<E>>;

/// Create a request handle.
///
/// Calling `run()` on this handle launches the request. When the `auto` argument is set to true,
/// the request is launched on mount.
#[hook]
pub fn use_request<R: GlooRequest + 'static, F: Fn(&Result<R::Response, R::Error>) + 'static>(
    prefix: Cow<'static, str>,
    request: R,
    after: F,
    auto: bool,
) -> UseRequestHandle<R::Response, R::Error>
where
    R::Response: Clone + 'static,
{
    use_async_with_options(
        async move {
            let result = request.send_prefix(&prefix).await;
            after(&result);
            result.map_err(Rc::new)
        },
        UseAsyncOptions { auto },
    )
}
