use gpui2::style::StyleHelpers;
use gpui2::{Element, IntoElement, ParentElement, ViewContext};
use strum::IntoEnumIterator;
use ui::{h_stack, label, prelude::*, v_stack};
use ui::{text_button, theme};

use crate::story::Story;

#[derive(Element, Default)]
pub struct TextButtonStory {}

impl TextButtonStory {
    fn render<V: 'static>(&mut self, _: &mut V, cx: &mut ViewContext<V>) -> impl IntoElement<V> {
        let theme = theme(cx);

        let states = InteractionState::iter();

        Story::container()
            .child(Story::title_for::<_, ui::TextButton>())
            .child(Story::label("Ghost (Default)"))
            .child(h_stack().gap_2().children(states.clone().map(|state| {
                let state_name = state.as_str().clone();

                v_stack()
                    .gap_1()
                    .child(
                        label(&state_name)
                            .color(ui::LabelColor::Muted)
                            .size(ui::LabelSize::Small),
                    )
                    .child(
                        text_button("Text Button")
                            .variant(ButtonVariant::Ghost)
                            .state(state),
                    )
            })))
            .child(Story::label("Filled"))
            .child(h_stack().gap_2().children(states.clone().map(|state| {
                let state_name = state.as_str().clone();

                v_stack()
                    .gap_1()
                    .child(
                        label(&state_name)
                            .color(ui::LabelColor::Muted)
                            .size(ui::LabelSize::Small),
                    )
                    .child(
                        text_button("Text Button")
                            .variant(ButtonVariant::Filled)
                            .state(state),
                    )
            })))
    }
}
