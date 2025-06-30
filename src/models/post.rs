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
        let mut paragraphs: Vec<String> = vec![];
        let mut start_index: usize = 0;
        let mut content = self.content.clone();

        loop {
            if content.len() < 2 {
                break;
            };

            if &content[content.len() - 2..content.len()] == "\\n" {
                content = content[0..content.len() - 2].to_string();
            } else {
                break;
            }
        }

        let chars = content.chars().collect::<Vec<char>>();

        for i in 0..chars.len() {
            if i < chars.len() - 1 {
                if chars[i] == '\\' && chars[i + 1] == 'n' {
                    if &content[start_index..i] != "" {
                        paragraphs.push(content[start_index..i].to_string());
                    }

                    start_index = i + 2;
                }
            }

            if i == chars.len() - 1 {
                paragraphs.push(content[start_index..chars.len()].to_string());
            }
        }

        if paragraphs.len() == 0 {
            paragraphs.push(content);
        }

        paragraphs
    }
}
