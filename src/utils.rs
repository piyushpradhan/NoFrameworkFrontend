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

    pub fn build(self) -> HtmlElement {
        self.element
    }
}
