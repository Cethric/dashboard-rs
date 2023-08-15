#![feature(custom_test_frameworks)]

#[cfg(test)]
mod tests {
    use leptos::*;
    use test_case::test_case;

    use components::class_name::ClassName;
    use components::data_display::card::*;
    use components::responsive::{Responsive, ResponsiveVec};

    #[test_case(false; "no border")]
    #[test_case(true; "with border")]
    pub fn test_card(bordered: bool) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx,
                <Card bordered=bordered>
                    <CardTitle slot>Title</CardTitle>
                    <CardBody slot>Body</CardBody>
                    <CardActions slot>Actions</CardActions>
                </Card>
            };
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-card-start--><!--leptos-view|components-src-data_display-card.rs-124|open--><div class=\"card{} w-96 bg-base-100 shadow-xl\" id=\"_0-2\"><!--hk=_0-3c|leptos-unit--><div class=\"card-body\" id=\"_0-4\"><h2 class=\"card-title\" id=\"_0-5\"><!--leptos-view|<CardTitle/>-children|open--><!--hk=_0-6o|leptos--start-->Title<!--hk=_0-6c|leptos--end--><!--leptos-view|<CardTitle/>-children|close--></h2><!--leptos-view|<CardBody/>-children|open--><!--hk=_0-7o|leptos--start-->Body<!--hk=_0-7c|leptos--end--><!--leptos-view|<CardBody/>-children|close--><div class=\"card-actions justify-end\" id=\"_0-8\"><!--leptos-view|<CardActions/>-children|open--><!--hk=_0-9o|leptos--start-->Actions<!--hk=_0-9c|leptos--end--><!--leptos-view|<CardActions/>-children|close--></div></div><!--hk=_0-10c|leptos-unit--></div><!--leptos-view|components-src-data_display-card.rs-124|close--><!--hk=_0-1c|leptos-card-end-->",
                    if bordered {" card-bordered"} else {""}
                )
            );
        }).dispose();
    }

    #[test_case(CardPadding::Normal; "Normal")]
    #[test_case(CardPadding::Compact; "Compact")]
    pub fn test_card_padding(padding: CardPadding) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx,
                <Card padding=ResponsiveVec(vec![Responsive::Default(padding), Responsive::Small(padding), Responsive::Medium(padding), Responsive::Large(padding), Responsive::ExtraLarge(padding), Responsive::ExtraExtraLarge(padding)])>
                    <CardTitle slot>Title</CardTitle>
                    <CardBody slot>Body</CardBody>
                    <CardActions slot>Actions</CardActions>
                </Card>
            };
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-card-start--><!--leptos-view|components-src-data_display-card.rs-124|open--><div class=\"card{}{}{}{}{}{} w-96 bg-base-100 shadow-xl\" id=\"_0-2\"><!--hk=_0-3c|leptos-unit--><div class=\"card-body\" id=\"_0-4\"><h2 class=\"card-title\" id=\"_0-5\"><!--leptos-view|<CardTitle/>-children|open--><!--hk=_0-6o|leptos--start-->Title<!--hk=_0-6c|leptos--end--><!--leptos-view|<CardTitle/>-children|close--></h2><!--leptos-view|<CardBody/>-children|open--><!--hk=_0-7o|leptos--start-->Body<!--hk=_0-7c|leptos--end--><!--leptos-view|<CardBody/>-children|close--><div class=\"card-actions justify-end\" id=\"_0-8\"><!--leptos-view|<CardActions/>-children|open--><!--hk=_0-9o|leptos--start-->Actions<!--hk=_0-9c|leptos--end--><!--leptos-view|<CardActions/>-children|close--></div></div><!--hk=_0-10c|leptos-unit--></div><!--leptos-view|components-src-data_display-card.rs-124|close--><!--hk=_0-1c|leptos-card-end-->",
                    padding,
                    format!(" sm:{}", padding.class_name()),
                    format!(" md:{}", padding.class_name()),
                    format!(" lg:{}", padding.class_name()),
                    format!(" xl:{}", padding.class_name()),
                    format!(" 2xl:{}", padding.class_name())
                )
            );
        }).dispose();
    }

    #[test_case(CardSide::Side; "Side")]
    pub fn test_card_side(side: CardSide) {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx,
                <Card side=ResponsiveVec(vec![Responsive::Default(side), Responsive::Small(side), Responsive::Medium(side), Responsive::Large(side), Responsive::ExtraLarge(side), Responsive::ExtraExtraLarge(side)])>
                    <CardTitle slot>Title</CardTitle>
                    <CardBody slot>Body</CardBody>
                    <CardActions slot>Actions</CardActions>
                </Card>
            };
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-card-start--><!--leptos-view|components-src-data_display-card.rs-124|open--><div class=\"card{}{}{}{}{}{} w-96 bg-base-100 shadow-xl\" id=\"_0-2\"><!--hk=_0-3c|leptos-unit--><div class=\"card-body\" id=\"_0-4\"><h2 class=\"card-title\" id=\"_0-5\"><!--leptos-view|<CardTitle/>-children|open--><!--hk=_0-6o|leptos--start-->Title<!--hk=_0-6c|leptos--end--><!--leptos-view|<CardTitle/>-children|close--></h2><!--leptos-view|<CardBody/>-children|open--><!--hk=_0-7o|leptos--start-->Body<!--hk=_0-7c|leptos--end--><!--leptos-view|<CardBody/>-children|close--><div class=\"card-actions justify-end\" id=\"_0-8\"><!--leptos-view|<CardActions/>-children|open--><!--hk=_0-9o|leptos--start-->Actions<!--hk=_0-9c|leptos--end--><!--leptos-view|<CardActions/>-children|close--></div></div><!--hk=_0-10c|leptos-unit--></div><!--leptos-view|components-src-data_display-card.rs-124|close--><!--hk=_0-1c|leptos-card-end-->",
                    side,
                    format!(" sm:{}", side.class_name()),
                    format!(" md:{}", side.class_name()),
                    format!(" lg:{}", side.class_name()),
                    format!(" xl:{}", side.class_name()),
                    format!(" 2xl:{}", side.class_name())
                )
            );
        }).dispose();
    }

    #[test]
    pub fn test_card_image_default() {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx,
                <Card>
                    <CardTitle slot>Title</CardTitle>
                    <CardBody slot>Body</CardBody>
                    <CardActions slot>Actions</CardActions>
                    <CardImage slot position=CardImagePosition::Default>Image</CardImage>
                </Card>
            };
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-card-start--><!--leptos-view|components-src-data_display-card.rs-124|open--><div class=\"card w-96 bg-base-100 shadow-xl\" id=\"_0-2\"><!--leptos-view|components-src-data_display-card.rs-118|open--><figure id=\"_0-3\"><!--leptos-view|<CardImage/>-children|open--><!--hk=_0-4o|leptos--start-->Image<!--hk=_0-4c|leptos--end--><!--leptos-view|<CardImage/>-children|close--></figure><!--leptos-view|components-src-data_display-card.rs-118|close--><div class=\"card-body\" id=\"_0-5\"><h2 class=\"card-title\" id=\"_0-6\"><!--leptos-view|<CardTitle/>-children|open--><!--hk=_0-7o|leptos--start-->Title<!--hk=_0-7c|leptos--end--><!--leptos-view|<CardTitle/>-children|close--></h2><!--leptos-view|<CardBody/>-children|open--><!--hk=_0-8o|leptos--start-->Body<!--hk=_0-8c|leptos--end--><!--leptos-view|<CardBody/>-children|close--><div class=\"card-actions justify-end\" id=\"_0-9\"><!--leptos-view|<CardActions/>-children|open--><!--hk=_0-10o|leptos--start-->Actions<!--hk=_0-10c|leptos--end--><!--leptos-view|<CardActions/>-children|close--></div></div><!--hk=_0-11c|leptos-unit--></div><!--leptos-view|components-src-data_display-card.rs-124|close--><!--hk=_0-1c|leptos-card-end-->",
                )
            );
        }).dispose();
    }

    #[test]
    pub fn test_card_image_top() {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx,
                <Card>
                    <CardTitle slot>Title</CardTitle>
                    <CardBody slot>Body</CardBody>
                    <CardActions slot>Actions</CardActions>
                    <CardImage slot position=CardImagePosition::Top>Image</CardImage>
                </Card>
            };
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-card-start--><!--leptos-view|components-src-data_display-card.rs-124|open--><div class=\"card w-96 bg-base-100 shadow-xl\" id=\"_0-2\"><!--leptos-view|components-src-data_display-card.rs-118|open--><figure id=\"_0-3\"><!--leptos-view|<CardImage/>-children|open--><!--hk=_0-4o|leptos--start-->Image<!--hk=_0-4c|leptos--end--><!--leptos-view|<CardImage/>-children|close--></figure><!--leptos-view|components-src-data_display-card.rs-118|close--><div class=\"card-body\" id=\"_0-5\"><h2 class=\"card-title\" id=\"_0-6\"><!--leptos-view|<CardTitle/>-children|open--><!--hk=_0-7o|leptos--start-->Title<!--hk=_0-7c|leptos--end--><!--leptos-view|<CardTitle/>-children|close--></h2><!--leptos-view|<CardBody/>-children|open--><!--hk=_0-8o|leptos--start-->Body<!--hk=_0-8c|leptos--end--><!--leptos-view|<CardBody/>-children|close--><div class=\"card-actions justify-end\" id=\"_0-9\"><!--leptos-view|<CardActions/>-children|open--><!--hk=_0-10o|leptos--start-->Actions<!--hk=_0-10c|leptos--end--><!--leptos-view|<CardActions/>-children|close--></div></div><!--hk=_0-11c|leptos-unit--></div><!--leptos-view|components-src-data_display-card.rs-124|close--><!--hk=_0-1c|leptos-card-end-->",
                )
            );
        }).dispose();
    }

    #[test]
    pub fn test_card_image_bottom() {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx,
                <Card>
                    <CardTitle slot>Title</CardTitle>
                    <CardBody slot>Body</CardBody>
                    <CardActions slot>Actions</CardActions>
                    <CardImage slot position=CardImagePosition::Bottom>Image</CardImage>
                </Card>
            };
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-card-start--><!--leptos-view|components-src-data_display-card.rs-124|open--><div class=\"card w-96 bg-base-100 shadow-xl\" id=\"_0-2\"><!--hk=_0-3c|leptos-unit--><div class=\"card-body\" id=\"_0-4\"><h2 class=\"card-title\" id=\"_0-5\"><!--leptos-view|<CardTitle/>-children|open--><!--hk=_0-6o|leptos--start-->Title<!--hk=_0-6c|leptos--end--><!--leptos-view|<CardTitle/>-children|close--></h2><!--leptos-view|<CardBody/>-children|open--><!--hk=_0-7o|leptos--start-->Body<!--hk=_0-7c|leptos--end--><!--leptos-view|<CardBody/>-children|close--><div class=\"card-actions justify-end\" id=\"_0-8\"><!--leptos-view|<CardActions/>-children|open--><!--hk=_0-9o|leptos--start-->Actions<!--hk=_0-9c|leptos--end--><!--leptos-view|<CardActions/>-children|close--></div></div><!--leptos-view|components-src-data_display-card.rs-118|open--><figure id=\"_0-10\"><!--leptos-view|<CardImage/>-children|open--><!--hk=_0-11o|leptos--start-->Image<!--hk=_0-11c|leptos--end--><!--leptos-view|<CardImage/>-children|close--></figure><!--leptos-view|components-src-data_display-card.rs-118|close--></div><!--leptos-view|components-src-data_display-card.rs-124|close--><!--hk=_0-1c|leptos-card-end-->",
                )
            );
        }).dispose();
    }

    #[test]
    pub fn test_card_image_full() {
        create_scope(create_runtime(), move |cx| {
            let view = view! {cx,
                <Card>
                    <CardTitle slot>Title</CardTitle>
                    <CardBody slot>Body</CardBody>
                    <CardActions slot>Actions</CardActions>
                    <CardImage slot position=CardImagePosition::Full>Image</CardImage>
                </Card>
            };
            assert_eq!(
                view.render_to_string(cx),
                format!(
                    "<!--hk=_0-1o|leptos-card-start--><!--leptos-view|components-src-data_display-card.rs-124|open--><div class=\"card image-full w-96 bg-base-100 shadow-xl\" id=\"_0-2\"><!--leptos-view|components-src-data_display-card.rs-118|open--><figure id=\"_0-3\"><!--leptos-view|<CardImage/>-children|open--><!--hk=_0-4o|leptos--start-->Image<!--hk=_0-4c|leptos--end--><!--leptos-view|<CardImage/>-children|close--></figure><!--leptos-view|components-src-data_display-card.rs-118|close--><div class=\"card-body\" id=\"_0-5\"><h2 class=\"card-title\" id=\"_0-6\"><!--leptos-view|<CardTitle/>-children|open--><!--hk=_0-7o|leptos--start-->Title<!--hk=_0-7c|leptos--end--><!--leptos-view|<CardTitle/>-children|close--></h2><!--leptos-view|<CardBody/>-children|open--><!--hk=_0-8o|leptos--start-->Body<!--hk=_0-8c|leptos--end--><!--leptos-view|<CardBody/>-children|close--><div class=\"card-actions justify-end\" id=\"_0-9\"><!--leptos-view|<CardActions/>-children|open--><!--hk=_0-10o|leptos--start-->Actions<!--hk=_0-10c|leptos--end--><!--leptos-view|<CardActions/>-children|close--></div></div><!--hk=_0-11c|leptos-unit--></div><!--leptos-view|components-src-data_display-card.rs-124|close--><!--hk=_0-1c|leptos-card-end-->",
                )
            );
        }).dispose();
    }
}
