use std::iter;
use rand::Rng;

pub struct PasswordGeneratorBuilder {
    lowercase: bool,
    uppercase: bool,
    numbers: bool,
    special: bool,
}

impl PasswordGeneratorBuilder {
    pub fn new() -> Self {
        PasswordGeneratorBuilder {
            lowercase: false,
            uppercase: false,
            numbers: false,
            special: false,
        }
    }

    pub fn with_lowercase(mut self, lowercase: bool) -> Self {
        self.lowercase = lowercase;
        self
    }

    pub fn with_uppercase(mut self, uppercase: bool) -> Self {
        self.uppercase = uppercase;
        self
    }

    pub fn with_numbers(mut self, numbers: bool) -> Self {
        self.numbers = numbers;
        self
    }

    pub fn with_special(mut self, special: bool) -> Self {
        self.special = special;
        self
    }

    fn create_charset(&self) -> Vec<char> {
        let mut charset: Vec<char> = Vec::new();

        if self.lowercase {
            charset.extend('a'..'z');
        }

        if self.uppercase {
            charset.extend('A'..'Z');
        }

        if self.numbers {
            charset.extend('0'..'9');
        }

        if self.special {
            charset.extend(vec!['!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '+', '=', '{', '}', '[', ']', '\\']);
        }

        charset
    }

    pub fn build(self) -> PasswordGenerator {
        PasswordGenerator {
            charset: self.create_charset(),
        }
    }
}

pub struct PasswordGenerator {
    charset: Vec<char>,
}

impl PasswordGenerator {
    pub fn generate_password(&self, length: usize) -> String {
        let charset = &self.charset;

        iter::repeat(())
            .map(|()| rand::thread_rng().gen_range(0..charset.len()))
            .map(|i| charset[i])
            .take(length)
            .collect()
    }
}

fn main() {
    let generator = PasswordGeneratorBuilder::new()
        .with_lowercase(true)
        .with_uppercase(true)
        .with_numbers(true)
        .with_special(true)
        .build();

    let passwords: Vec<String> = (0..10) // <- password count
        .into_iter()
        .map(|_| generator.generate_password(16))
        .collect();

    for password in passwords {
        println!("Generated password: {}", password);
    }
}