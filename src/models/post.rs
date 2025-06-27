use std::usize;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl Post {
    pub fn get_date(&self) -> String {
        self.created_at.format("%m/%d/%y").to_string()
    }

    pub fn get_paragraphs(&self) -> Vec<String> {
        let chars = self.content.chars().collect::<Vec<char>>();
        let mut paragraphs: Vec<String> = vec![];
        let mut start_index: usize = 0;

        for (index, char) in self.content.chars().enumerate() {
            if index < chars.len() + 2 {
                if char == '\\' && chars[index + 1] == 'n' {
                    if index < chars.len() + 3 {
                        if chars[index + 2] != '\\' && chars[index + 3] != 'n' {
                            paragraphs.push(self.content[start_index..index].to_string());
                        }
                    }
                    start_index = index + 2;
                }
            }

            if index == chars.len() - 1 {
                paragraphs.push(self.content[start_index..index].to_string());
            }
        }

        if paragraphs.len() == 0 {
            paragraphs.push(self.content.clone());
        }

        paragraphs
    }
}
