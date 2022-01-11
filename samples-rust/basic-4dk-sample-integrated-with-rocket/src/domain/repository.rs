use better_any::Tid;

pub trait Repository {
    fn as_tid(&self) -> &dyn Tid;
}