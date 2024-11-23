use web_sys::HtmlElement;

// Component trait for defining components
pub trait Component {
    fn render(&self) -> HtmlElement;
}
