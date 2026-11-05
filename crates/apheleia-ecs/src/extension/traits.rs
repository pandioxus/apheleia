use std::{any::Any, fmt::Debug, sync::{OnceLock, atomic::AtomicUsize}};

static COMPONENT_ID_COUNT: AtomicUsize = AtomicUsize::new(0);
pub trait Extension: Any + Debug + 'static {
    fn id() -> usize
    where
        Self: Sized,
    {
        static ID: OnceLock<usize> = OnceLock::new();
        *ID.get_or_init(|| {
            COMPONENT_ID_COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        })
    }
}
