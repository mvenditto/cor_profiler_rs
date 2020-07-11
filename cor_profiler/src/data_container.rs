
use std::rc::Rc;

pub(crate) trait DataItem {
    
}
pub(crate) trait DataContainer<T: DataItem + ?Sized> {
    fn set_item(&self, key: String, item: Rc<T>);

    fn get_item(&self, key: String) -> Option<Rc<T>>;
}
