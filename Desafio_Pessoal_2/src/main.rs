use druid::widget::{Button, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Lens, LocalizedString, Widget, WidgetExt, WindowDesc};

// Definindo o estado da aplicação
#[derive(Clone, Data, Lens)]
struct AppState {
    name: String,
    screen: Screen,
}

// Definindo as diferentes telas
#[derive(Clone, Data, PartialEq)]
enum Screen {
    Input,
    Pibic,
}

fn text_pibic() -> impl Widget<AppState> {
    let label = Label::new("PIBIC").center();
    Flex::column()
        .with_flex_spacer(1.0)
        .with_child(label)
        .with_flex_spacer(1.0)
}

fn input_screen() -> impl Widget<AppState> {
    let label = Label::new("Olá, insira seu nome:");
    let textbox = TextBox::new()
        .with_placeholder("Insira seu nome")
        .lens(AppState::name);
    let name_label = Label::new(|data: &AppState, _env: &_| format!("Olá, {}!", data.name));
    let button = Button::new("Okay").on_click(|_ctx, data: &mut AppState, _env| {
        data.screen = Screen::Pibic;
    });

    Flex::column()
        .with_child(label)
        .with_child(textbox)
        .with_spacer(8.0)
        .with_child(name_label)
        .with_spacer(8.0)
        .with_child(button)
        .padding(10.0)
}

fn build_ui() -> impl Widget<AppState> {
    let input_screen = input_screen();
    let text_pibic = text_pibic();

    // Uma função que seleciona o widget com base no estado
    druid::widget::Either::new(
        |data, _| data.screen == Screen::Input,
        input_screen,
        text_pibic,
    )
}

fn main() {
    let main_window = WindowDesc::new(build_ui()).title(LocalizedString::new("Hello Druid"));

    let initial_state = AppState {
        name: String::new(),
        screen: Screen::Input,
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application.");
}
