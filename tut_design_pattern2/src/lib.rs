pub trait Foo {
    fn info(&self) {
        println!("Info::()");
    }
}

pub struct Post {
    content: String
}

impl Foo for Post {}

impl Post {
    pub fn new() -> Draft {
        return Draft {
            content: String::new()
        };
    }

    pub fn content(&self) -> &String {
        return &self.content;
    }
}

pub struct Draft {
    content: String
}

impl Foo for Draft {}

impl Draft {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn review(self) -> Review {
        return Review {
            content: self.content
        };
    }
}

pub struct Review {
    content: String
}

impl Foo for Review {}

impl Review {
    pub fn approve(self) -> Post {
        return Post {
            content: self.content
        };
    }
}