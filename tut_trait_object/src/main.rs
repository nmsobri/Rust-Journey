pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    components: Vec<Box<Clone>>
}

impl Screen {
    pub fn new() -> Self {
        Self {
            components: vec![]
        }
    }

    pub fn add(&mut self, item: Box<Draw>) {
        self.components.push(item);
    }

    pub fn run(&self) {
        for item in self.components.iter() {
            item.draw();
        }
    }
}

pub struct Button {
    x: i32,
    y: i32,
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing for Button, label:{}, x:{}, y:{}", self.label, self.x, self.y);
    }
}

pub struct TextBox {
    x: i32,
    y: i32,
    placeholder: String,
}

impl Draw for TextBox {
    fn draw(&self) {
        println!("Drawing for TextButton, placeholder:{}, x:{}, y:{}", self.placeholder, self.x, self.y);
    }
}

fn main() {
    let mut screen = Screen::new();

    screen.add(Box::new(TextBox { x: 1, y: 2, placeholder: "This is textbox".to_string() }));
    screen.add(Box::new(Button { x: 1, y: 2, label: "This is textbox".to_string() }));
    screen.run();
}
