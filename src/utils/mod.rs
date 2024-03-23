pub mod api;
pub mod storage;

use wasm_bindgen::prelude::*;
use web_sys::{window, HtmlElement};

pub struct Element {
    pub element: HtmlElement,
}

pub trait HtmlElementMethod {
    fn call(self, element: &HtmlElement);
}

impl<F> HtmlElementMethod for F
where
    F: FnOnce(&HtmlElement),
{
    fn call(self, element: &HtmlElement) {
        self(element);
    }
}

impl Element {
    pub fn new(name: &str) -> Self {
        let document = window().unwrap().document().unwrap();
        let element = document
            .create_element(name)
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();

        Element { element }
    }

    pub fn set<M>(&mut self, method: M) -> &mut Self
    where
        M: HtmlElementMethod,
    {
        let element = self.element.clone();
        method.call(&element);
        self
    }

    pub fn child(&mut self, child: HtmlElement) -> &mut Self {
        let _ = self.element.append_child(&child);
        self
    }

    pub fn build(&mut self) -> HtmlElement {
        let element = self.element.clone();
        element
    }
}
