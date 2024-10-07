// pub struct MenuDescriptor {
//     pub components: Vec<Component>,
// }
// impl MenuDescriptor {
//     pub fn new() -> Self {
//         MenuDescriptor {
//             components: Vec::new(),
//         }
//     }
// }
// impl MenuDescriptor {
//     pub fn from_menu_type(menu: MenuType, font_system: &mut FontSystem) -> Self {
//         let mut descriptor = MenuDescriptor::new();

//         match menu {
//             MenuType::Main => {
//
//                 descriptor.components.push(Component::Text(
//                     Id(0),
//                     text::Text::new(
//                         font_system,
//                         RectPos {
//                             top: 250,
//                             left: 100,
//                             bottom: 400,
//                             right: 400,
//                         },
//                         &format!("Success!"),
//                         Color::rgb(0, 200, 0),
//                         Color::rgb(0, 200, 0),
//                     ),
//                 ));

//
//                 descriptor.components.push(Component::Button(
//                     Id(1),
//                     Button::new(
//                         ButtonConfig {
//                             rect_pos: RectPos {
//                                 top: 200,
//                                 left: 250,
//                                 bottom: 250,
//                                 right: 550,
//                             },
//                             fill_color: [0.2, 0.3, 0.6],
//                             fill_color_active: [0.3, 0.4, 0.7],
//                             border_color: [0.0, 0.0, 0.0],
//                             border_color_active: [0.0, 0.0, 0.0],
//                             text: "Play",
//                             text_color: Color::rgb(255, 255, 255),
//                             text_color_active: Color::rgb(255, 255, 255),
//                             on_click: Box::new(|| log_debug!("Play button clicked")),
//                         },
//                         font_system,
//                     ),
//                 ));

//                 descriptor.components.push(Component::Button(
//                     Id(2),
//                     Button::new(
//                         ButtonConfig {
//                             rect_pos: RectPos {
//                                 top: 275,
//                                 left: 250,
//                                 bottom: 325,
//                                 right: 550,
//                             },
//                             fill_color: [0.2, 0.5, 0.2],
//                             fill_color_active: [0.3, 0.6, 0.3],
//                             border_color: [0.0, 0.0, 0.0],
//                             border_color_active: [0.0, 0.0, 0.0],
//                             text: "Settings",
//                             text_color: Color::rgb(255, 255, 255),
//                             text_color_active: Color::rgb(255, 255, 255),
//                             on_click: Box::new(|| log_debug!("Settings button clicked")),
//                         },
//                         font_system,
//                     ),
//                 ));

//                 descriptor.components.push(Component::Button(
//                     Id(3),
//                     Button::new(
//                         ButtonConfig {
//                             rect_pos: RectPos {
//                                 top: 350,
//                                 left: 250,
//                                 bottom: 400,
//                                 right: 550,
//                             },
//                             fill_color: [0.6, 0.2, 0.2],
//                             fill_color_active: [0.7, 0.3, 0.3],
//                             border_color: [0.0, 0.0, 0.0],
//                             border_color_active: [0.0, 0.0, 0.0],
//                             text: "Exit",
//                             text_color: Color::rgb(255, 255, 255),
//                             text_color_active: Color::rgb(255, 255, 255),
//                             on_click: Box::new(|| log_debug!("Exit button clicked")),
//                         },
//                         font_system,
//                     ),
//                 ));
//             }
//             MenuType::Settings => {
//
//                 descriptor.components.push(Component::Text(
//                     Id(0),
//                     text::Text::new(
//                         font_system,
//                         RectPos {
//                             top: 50,
//                             left: 200,
//                             bottom: 150,
//                             right: 600,
//                         },
//                         "Settings",
//                         Color::rgb(255, 255, 255),
//                         Color::rgb(0, 0, 0),
//                     ),
//                 ));

//
//                 descriptor.components.push(Component::TextField(
//                     Id(1),
//                     TextField::new(
//                         TextFieldConfig {
//                             rect_pos: RectPos {
//                                 top: 200,
//                                 left: 250,
//                                 bottom: 250,
//                                 right: 550,
//                             },
//                             fill_color: [0.9, 0.9, 0.9],
//                             fill_color_active: [1.0, 1.0, 1.0],
//                             border_color: [0.3, 0.3, 0.3],
//                             border_color_active: [0.1, 0.1, 0.1],
//                             text_color: Color::rgb(0, 0, 0),
//                         },
//                         font_system,
//                     ),
//                 ));
//             }
//             MenuType::ShaderSandbox => {
//                 descriptor.components.push(Component::Text(
//                     Id(0),
//                     text::Text::new(
//                         font_system,
//                         RectPos {
//                             top: 50,
//                             left: 200,
//                             bottom: 150,
//                             right: 600,
//                         },
//                         "Shader Sandbox",
//                         Color::rgb(255, 255, 255),
//                         Color::rgb(0, 0, 0),
//                     ),
//                 ));
//             }
//         }

//         descriptor
//     }
// }
