use web_sys::HtmlElement;

// Component trait for defining components
pub trait Component {
    fn render(&self) -> HtmlElement;
    fn on_mount(&mut self) {}
}
