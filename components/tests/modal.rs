#![feature(custom_test_frameworks)]

#[cfg(test)]
mod tests {
    use leptos::html::Dialog;
    use leptos::*;
    use test_case::test_case;

    use components::actions::button::*;
    use components::actions::modal::*;

    #[test_case(ModalPosition::Default; "Default")]
    #[test_case(ModalPosition::Top; "Top")]
    #[test_case(ModalPosition::Middle; "Middle")]
    #[test_case(ModalPosition::Bottom; "Bottom")]
    pub fn test_modal(position: ModalPosition) {
        create_scope(create_runtime(), move |cx| {
            let modal_ref = create_node_ref::<Dialog>(cx);
            let view = view! {cx,
                <Modal position=position modal_ref=modal_ref>
                    <h3 class="font-bold text-lg">Hello!</h3>
                    <p class="py-4">Press ESC key or click the button below to close</p>
                    <ModalActions slot>
                        <Button button_type=ButtonType::Submit>Submit</Button>
                        <Button on:click=move |_| { modal_ref.get().expect("Invalid dialog").close();}>Close</Button>
                    </ModalActions>
                    <ModalEntrance slot>
                        <Button on:click=move |_| { let _ = modal_ref.get().expect("Invalid dialog").show_modal();}>Open Modal</Button>
                    </ModalEntrance>
                </Modal>
            };
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-modal-start--><!--leptos-view|components-src-actions-modal.rs-56|open--><!--hk=_0-2o|leptos--start--><!--leptos-view|<ModalEntrance/>-children|open--><!--hk=_0-3o|leptos--start--><!--hk=_0-4o|leptos-button-start--><!--leptos-view|components-src-actions-button.rs-132|open--><button class=\"btn\" type id=\"_0-5\"><!--leptos-view|<Button/>-children|open--><!--hk=_0-6o|leptos--start-->Open Modal<!--hk=_0-6c|leptos--end--><!--leptos-view|<Button/>-children|close--></button><!--leptos-view|components-src-actions-button.rs-132|close--><!--hk=_0-4c|leptos-button-end--><!--hk=_0-3c|leptos--end--><!--leptos-view|<ModalEntrance/>-children|close--><dialog class=\"modal{}\" id=\"_0-7\"><form method=\"dialog\" class=\"modal-box\" id=\"_0-8\"><!--leptos-view|<Modal/>-children|open--><!--hk=_0-9o|leptos--start--><h3 class=\"font-bold text-lg\" id=\"_0-10\">Hello!</h3><p class=\"py-4\" id=\"_0-11\">Press ESC key or click the button below to close</p><!--hk=_0-9c|leptos--end--><!--leptos-view|<Modal/>-children|close--><!--leptos-view|components-src-actions-modal.rs-62|open--><div class=\"modal-action\" id=\"_0-12\"><!--leptos-view|<ModalActions/>-children|open--><!--hk=_0-13o|leptos--start--><!--hk=_0-14o|leptos-button-start--><!--leptos-view|components-src-actions-button.rs-132|open--><button class=\"btn\" type=\"submit\" id=\"_0-15\"><!--leptos-view|<Button/>-children|open--><!--hk=_0-16o|leptos--start-->Submit<!--hk=_0-16c|leptos--end--><!--leptos-view|<Button/>-children|close--></button><!--leptos-view|components-src-actions-button.rs-132|close--><!--hk=_0-14c|leptos-button-end--><!--hk=_0-17o|leptos-button-start--><!--leptos-view|components-src-actions-button.rs-132|open--><button class=\"btn\" type id=\"_0-18\"><!--leptos-view|<Button/>-children|open--><!--hk=_0-19o|leptos--start-->Close<!--hk=_0-19c|leptos--end--><!--leptos-view|<Button/>-children|close--></button><!--leptos-view|components-src-actions-button.rs-132|close--><!--hk=_0-17c|leptos-button-end--><!--hk=_0-13c|leptos--end--><!--leptos-view|<ModalActions/>-children|close--></div><!--leptos-view|components-src-actions-modal.rs-62|close--></form><form method=\"dialog\" class=\"modal-backdrop\" id=\"_0-20\"><button id=\"_0-21\">close</button></form></dialog><!--hk=_0-2c|leptos--end--><!--leptos-view|components-src-actions-modal.rs-56|close--><!--hk=_0-1c|leptos-modal-end-->",
                    position
                )
            );
        }).dispose();
    }
}
