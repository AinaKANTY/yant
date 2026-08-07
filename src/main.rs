use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{container, text, text_editor},
};

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("Yant")
        .run()
}

struct App {
    content: text_editor::Content,
    is_dirty: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                content: text_editor::Content::new(),
                is_dirty: false,
            },
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Edit(action) => {
                self.is_dirty = self.is_dirty || action.is_edit();

                self.content.perform(action);

                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input = text_editor(&self.content)
            .on_action(Message::Edit)
            .height(Fill)
            .wrapping(text::Wrapping::WordOrGlyph);

        container(input).padding(10).into()
    }
}
