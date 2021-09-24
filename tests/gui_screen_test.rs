use gui;

#[test]
fn run_test() {
    let screen = gui::Screen {
        components: vec![
            Box::new(gui::component::SelectBox {
                width: 75,
                height: 10,
                options: vec![String::from("yes"), String::from("no"), String::from("idk")],
            }),
            Box::new(gui::component::Button {
                width: 209,
                height: 239,

                label: String::from("Abiria"),
            }),
            Box::new(gui::component::TextField {
                place_holder: "Est grandis nomen, cesaris.".to_string(),
            }),
        ],
    };

    screen.run();
}
