pub trait State {
    fn review(self: Box<Self>) -> Box<State>;

    fn approve(self: Box<Self>) -> Box<State>;

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        return "";
    }
}

pub struct DraftState {}

impl State for DraftState {
    fn review(self: Box<Self>) -> Box<State> {
        return Box::new(ReviewState {});
    }

    fn approve(self: Box<Self>) -> Box<State> {
        return self;
    }
}

pub struct ReviewState {}

impl State for ReviewState {
    fn review(self: Box<Self>) -> Box<State> {
        return self;
    }

    fn approve(self: Box<Self>) -> Box<State> {
        return Box::new(PublishState {});
    }
}

pub struct PublishState {}

impl State for PublishState {
    fn review(self: Box<Self>) -> Box<State> {
        return self;
    }

    fn approve(self: Box<Self>) -> Box<State> {
        return self;
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        return &post.content;
    }
}


pub struct Post {
    content: String,
    state: Option<Box<State>>, //trait object
}

impl Post {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            state: Some(Box::new(DraftState {})),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        return self.state.as_ref().unwrap().content(&self);
    }

    pub fn review(&mut self) {
        if let Some(v) = self.state.take() {
            self.state = Some(v.review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(v) = self.state.take() {
            self.state = Some(v.approve());
        }
    }
}
