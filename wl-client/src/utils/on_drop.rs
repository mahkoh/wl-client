use run_on_drop::on_drop;

pub(crate) fn abort_on_panic<T>(f: impl FnOnce() -> T) -> T {
    let abort = on_drop(|| std::process::abort());
    let res = f();
    abort.forget();
    res
}
