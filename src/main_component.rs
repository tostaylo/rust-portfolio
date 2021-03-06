use crate::content::Content;
use crate::handle;
use crate::theme_component::ThemeComponent;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct MainState {
    show_themes: bool,
}

#[derive(Debug, Default, Clone)]
pub struct Main {
    id: String,
    state: MainState,
    child_content_component: handle::Handle<Content>,
    child_theme_component: handle::Handle<ThemeComponent>,
}

impl Main {
    pub fn create() -> handle::Handle<Self> {
        let main = Main {
            id: "main".to_owned(),
            child_content_component: Content::create(),
            child_theme_component: ThemeComponent::create(),
            ..Default::default()
        };
        handle::Handle(Rc::new(RefCell::new(main)))
    }
}

impl rust_fel::Component for handle::Handle<Main> {
    type Properties = Option<String>;
    type Message = Option<String>;
    type State = MainState;

    fn add_props(&mut self, _props: Self::Properties) {}

    fn reduce_state(&mut self, _message: Self::Message) {}

    fn render(&self) -> rust_fel::Element {
        let borrow = self.0.borrow_mut();
        let child_content_component = borrow.child_content_component.render();
        let theme_component = borrow.child_theme_component.render();

        let children = vec![theme_component, child_content_component];

        rust_fel::Element::new(
            "main".to_owned(),
            rust_fel::Props {
                id: Some(borrow.id.clone()),
                class_name: Some("main dark".to_owned()),
                children: Some(children),
                ..Default::default()
            },
        )
    }
}
