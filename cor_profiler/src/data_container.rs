
use std::rc::Rc;

pub(crate) trait DataItem {
    
}
pub(crate) trait DataContainer {
    fn set_item(&self, key: String, item: Rc<dyn DataItem>);

    fn get_item(&self, key: String) -> Option<Rc<dyn DataItem>>;
}
