use std::rc::Rc;
use syn::{parse_quote, TypePath};

#[cfg_attr(feature = "syn-debug", derive(Debug))]
#[derive(Clone)]
pub(crate) struct SharedAttrs {
    pub static_assertions_path: Rc<TypePath>,
    pub traits_path: Rc<TypePath>,
}
impl SharedAttrs {
    pub(crate) fn static_assertions_path(&self) -> Rc<TypePath> {
        self.static_assertions_path.clone()
    }
    pub(crate) fn traits_path(&self) -> Rc<TypePath> {
        self.traits_path.clone()
    }
}
impl Default for SharedAttrs {
    fn default() -> Self {
        Self {
            static_assertions_path: Rc::new(parse_quote!(::lib_conf::__private::static_assertions)),
            traits_path: Rc::new(parse_quote!(::lib_conf::traits)),
        }
    }
}
