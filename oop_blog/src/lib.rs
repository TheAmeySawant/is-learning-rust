use std::env::current_exe;

pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str;
    fn add_text(&self, post: &mut Post, text: &str);
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text(&self, post: &mut Post, text: &str) {}
}

struct PendingReview {
    approve_call_count: i8,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        if self.approve_call_count == 2 {
            return Box::new(Published {});
        } else if self.approve_call_count == 1 {
            self.approve_call_count += 1;
        }
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }

    fn add_text(&self, post: &mut Post, text: &str) {}
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {
            approve_call_count: 1,
        })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn add_text(&self, post: &mut Post, text: &str) {
        post.content.push_str(text);
    }
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Self {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if let Some(current_state) = self.state.take() {
            current_state.add_text(self, text);
            self.state = Some(current_state)
        }
    }

    pub fn request_review(&mut self) {
        if let Some(current_state) = self.state.take() {
            self.state = Some(current_state.request_review());
        }
    }

    pub fn reject(&mut self) {
        if let Some(current_state) = self.state.take() {
            self.state = Some(current_state.request_review());
        }
    }

    pub fn content<'a>(&'a self) -> &'a str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn approve(&mut self) {
        if let Some(current_state) = self.state.take() {
            self.state = Some(current_state.approve());
        }
    }
}

pub enum EnState {
    Draft,
    PendingReview(i8),
    Published,
}

pub struct EnPost {
    state: EnState,
    content: String,
}

impl EnPost {
    pub fn new() -> Self {
        EnPost {
            state: EnState::Draft,
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        match self.state {
            EnState::Draft => self.content.push_str(text),
            _ => {}
        }
    }

    pub fn request_review(&mut self) {
        match self.state {
            EnState::Draft => self.state = EnState::PendingReview(1),
            _ => {}
        }
    }

    pub fn approve(&mut self) {
        match self.state {
            EnState::PendingReview(ref mut x) => {
                if *x == 1 {
                    *x = 2;
                } else if *x == 2 {
                    self.state = EnState::Published
                }
            }
            _ => {}
        }
    }

    pub fn reject(&mut self) {
        match self.state {
            EnState::PendingReview(x) => self.state = EnState::Draft,
            _ => {}
        }
    }

    pub fn content<'a>(&'a self) -> &'a str {
        match self.state {
            EnState::Published => &self.content,
            _ => "",
        }
    }
}
