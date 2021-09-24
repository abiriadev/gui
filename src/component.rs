use crate::Draw;

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        //
    }
}

pub struct TextField {
    pub place_holder: String,
}

impl Draw for TextField {
    fn draw(&self) {
        //
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {}
}
