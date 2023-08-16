#![feature(custom_test_frameworks)]

#[cfg(test)]
mod tests {
    use leptos::*;
    use test_case::test_case;
    use uuid::Uuid;

    use components::data_display::carousel::*;

    #[test]
    pub fn test_carousel_indicator() {
        create_scope(create_runtime(), move |cx| {
            let id = Uuid::new_v4().to_string();
            let view = view! {cx, <Carousel id=id.to_string() indicator=true><CarouselItem slot>item 1</CarouselItem><CarouselItem slot>item 2</CarouselItem></Carousel>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-carousel-start--><!--leptos-view|components-src-data_display-carousel.rs-97|open--><!--hk=_0-5o|leptos--start--><div class=\"carousel\" id=\"_0-6\"><!--hk=_0-19o|leptos--start--><!--leptos-view|components-src-data_display-carousel.rs-86|open--><div class=\"carousel-item\" id=\"{}-item-0\" leptos-hk=\"_0-7\"><!--leptos-view|<CarouselItem/>-children|open--><!--hk=_0-8o|leptos--start-->item 1<!--hk=_0-8c|leptos--end--><!--leptos-view|<CarouselItem/>-children|close-->\n                if prev_next <!--leptos-view|components-src-data_display-carousel.rs-89|open--><div class=\"absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2\" id=\"_0-9\"><a href=\"#{}-item-0\" class=\"btn btn-circle\" id=\"_0-10\">&lt;</a><a href=\"#{}-item-0\" class=\"btn btn-circle\" id=\"_0-11\">&gt;</a></div><!--leptos-view|components-src-data_display-carousel.rs-89|close--> else <!--hk=_0-12c|leptos-unit--></div><!--leptos-view|components-src-data_display-carousel.rs-86|close--><!--leptos-view|components-src-data_display-carousel.rs-86|open--><div class=\"carousel-item\" id=\"{}-item-1\" leptos-hk=\"_0-13\"><!--leptos-view|<CarouselItem/>-children|open--><!--hk=_0-14o|leptos--start-->item 2<!--hk=_0-14c|leptos--end--><!--leptos-view|<CarouselItem/>-children|close-->\n                if prev_next <!--leptos-view|components-src-data_display-carousel.rs-89|open--><div class=\"absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2\" id=\"_0-15\"><a href=\"#{}-item-1\" class=\"btn btn-circle\" id=\"_0-16\">&lt;</a><a href=\"#{}-item-1\" class=\"btn btn-circle\" id=\"_0-17\">&gt;</a></div><!--leptos-view|components-src-data_display-carousel.rs-89|close--> else <!--hk=_0-18c|leptos-unit--></div><!--leptos-view|components-src-data_display-carousel.rs-86|close--><!--hk=_0-19c|leptos--end--></div><!--leptos-view|components-src-data_display-carousel.rs-101|open--><div class=\"flex justify-center w-full py-2 gap-2\" id=\"_0-20\"><!--hk=_0-4o|leptos--start--><!--leptos-view|components-src-data_display-carousel.rs-81|open--><a href=\"#{}-item-0\" class=\"btn btn-xs\" id=\"_0-2\">0</a><!--leptos-view|components-src-data_display-carousel.rs-81|close--><!--leptos-view|components-src-data_display-carousel.rs-81|open--><a href=\"#{}-item-1\" class=\"btn btn-xs\" id=\"_0-3\">1</a><!--leptos-view|components-src-data_display-carousel.rs-81|close--><!--hk=_0-4c|leptos--end--></div><!--leptos-view|components-src-data_display-carousel.rs-101|close--><!--hk=_0-5c|leptos--end--><!--leptos-view|components-src-data_display-carousel.rs-97|close--><!--hk=_0-1c|leptos-carousel-end-->",
                    id,id,id,id,id,id,id,id
                )
            );
        }).dispose();
    }

    #[test_case(CarouselSnap::Default; "Default")]
    #[test_case(CarouselSnap::Start; "Start")]
    #[test_case(CarouselSnap::Center; "Center")]
    #[test_case(CarouselSnap::End; "End")]
    pub fn test_carousel_snap(snap: CarouselSnap) {
        create_scope(create_runtime(), move |cx| {
            let id = Uuid::new_v4().to_string();
            let view = view! {cx, <Carousel id=id.to_string() snap=snap><CarouselItem slot>item 1</CarouselItem><CarouselItem slot>item 2</CarouselItem></Carousel>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-carousel-start--><!--leptos-view|components-src-data_display-carousel.rs-97|open--><!--hk=_0-5o|leptos--start--><div class=\"carousel{}\" id=\"_0-6\"><!--hk=_0-19o|leptos--start--><!--leptos-view|components-src-data_display-carousel.rs-86|open--><div class=\"carousel-item\" id=\"{}-item-0\" leptos-hk=\"_0-7\"><!--leptos-view|<CarouselItem/>-children|open--><!--hk=_0-8o|leptos--start-->item 1<!--hk=_0-8c|leptos--end--><!--leptos-view|<CarouselItem/>-children|close-->\n                if prev_next <!--leptos-view|components-src-data_display-carousel.rs-89|open--><div class=\"absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2\" id=\"_0-9\"><a href=\"#{}-item-0\" class=\"btn btn-circle\" id=\"_0-10\">&lt;</a><a href=\"#{}-item-0\" class=\"btn btn-circle\" id=\"_0-11\">&gt;</a></div><!--leptos-view|components-src-data_display-carousel.rs-89|close--> else <!--hk=_0-12c|leptos-unit--></div><!--leptos-view|components-src-data_display-carousel.rs-86|close--><!--leptos-view|components-src-data_display-carousel.rs-86|open--><div class=\"carousel-item\" id=\"{}-item-1\" leptos-hk=\"_0-13\"><!--leptos-view|<CarouselItem/>-children|open--><!--hk=_0-14o|leptos--start-->item 2<!--hk=_0-14c|leptos--end--><!--leptos-view|<CarouselItem/>-children|close-->\n                if prev_next <!--leptos-view|components-src-data_display-carousel.rs-89|open--><div class=\"absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2\" id=\"_0-15\"><a href=\"#{}-item-1\" class=\"btn btn-circle\" id=\"_0-16\">&lt;</a><a href=\"#{}-item-1\" class=\"btn btn-circle\" id=\"_0-17\">&gt;</a></div><!--leptos-view|components-src-data_display-carousel.rs-89|close--> else <!--hk=_0-18c|leptos-unit--></div><!--leptos-view|components-src-data_display-carousel.rs-86|close--><!--hk=_0-19c|leptos--end--></div><!--hk=_0-20c|leptos-unit--><!--hk=_0-5c|leptos--end--><!--leptos-view|components-src-data_display-carousel.rs-97|close--><!--hk=_0-1c|leptos-carousel-end-->",
                    snap,
                    id,id,id,id,id,id
                )
            );
        }).dispose();
    }

    #[test_case(CarouselOrientation::Default; "Default")]
    #[test_case(CarouselOrientation::Horizontal; "Horizontal")]
    #[test_case(CarouselOrientation::Vertical; "Vertical")]
    pub fn test_carousel_orientation(orientation: CarouselOrientation) {
        create_scope(create_runtime(), move |cx| {
            let id = Uuid::new_v4().to_string();
            let view = view! {cx, <Carousel id=id.to_string() orientation=orientation><CarouselItem slot>item 1</CarouselItem><CarouselItem slot>item 2</CarouselItem></Carousel>};
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-carousel-start--><!--leptos-view|components-src-data_display-carousel.rs-97|open--><!--hk=_0-5o|leptos--start--><div class=\"carousel{}\" id=\"_0-6\"><!--hk=_0-19o|leptos--start--><!--leptos-view|components-src-data_display-carousel.rs-86|open--><div class=\"carousel-item\" id=\"{}-item-0\" leptos-hk=\"_0-7\"><!--leptos-view|<CarouselItem/>-children|open--><!--hk=_0-8o|leptos--start-->item 1<!--hk=_0-8c|leptos--end--><!--leptos-view|<CarouselItem/>-children|close-->\n                if prev_next <!--leptos-view|components-src-data_display-carousel.rs-89|open--><div class=\"absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2\" id=\"_0-9\"><a href=\"#{}-item-0\" class=\"btn btn-circle\" id=\"_0-10\">&lt;</a><a href=\"#{}-item-0\" class=\"btn btn-circle\" id=\"_0-11\">&gt;</a></div><!--leptos-view|components-src-data_display-carousel.rs-89|close--> else <!--hk=_0-12c|leptos-unit--></div><!--leptos-view|components-src-data_display-carousel.rs-86|close--><!--leptos-view|components-src-data_display-carousel.rs-86|open--><div class=\"carousel-item\" id=\"{}-item-1\" leptos-hk=\"_0-13\"><!--leptos-view|<CarouselItem/>-children|open--><!--hk=_0-14o|leptos--start-->item 2<!--hk=_0-14c|leptos--end--><!--leptos-view|<CarouselItem/>-children|close-->\n                if prev_next <!--leptos-view|components-src-data_display-carousel.rs-89|open--><div class=\"absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2\" id=\"_0-15\"><a href=\"#{}-item-1\" class=\"btn btn-circle\" id=\"_0-16\">&lt;</a><a href=\"#{}-item-1\" class=\"btn btn-circle\" id=\"_0-17\">&gt;</a></div><!--leptos-view|components-src-data_display-carousel.rs-89|close--> else <!--hk=_0-18c|leptos-unit--></div><!--leptos-view|components-src-data_display-carousel.rs-86|close--><!--hk=_0-19c|leptos--end--></div><!--hk=_0-20c|leptos-unit--><!--hk=_0-5c|leptos--end--><!--leptos-view|components-src-data_display-carousel.rs-97|close--><!--hk=_0-1c|leptos-carousel-end-->",
                    orientation,
                    id,id,id,id,id,id
                )
            );
        }).dispose();
    }
}
