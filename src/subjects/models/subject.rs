// interface Subject {
// id: string,
// instructorId: string,
// title: string,
// grade: number,
// term: number,
// }

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Debug, FromRow)]
pub struct Subject {
    pub id: String,
    #[serde(rename="instructorId")]
    pub instructor_id: String,
    pub title: String,
    pub grade: i32,
    pub term: i32
}

#[derive(Deserialize)]
pub struct SubjectNew {
    #[serde(rename="instructorId")]
    pub instructor_id: String,
    pub title: String,
    pub grade: i32,
    pub term: i32
}

impl Subject {
    pub fn new(subject_new: &SubjectNew) -> Self {
        let id :String = Self::gen_id(&subject_new.title, subject_new.grade, subject_new.term);
        Subject {
            id,
            instructor_id: subject_new.instructor_id.clone(),
            title: subject_new.title.clone(),
            grade: subject_new.grade,
            term: subject_new.term
        }
    }

    fn gen_id(title: &String, grade: i32, term: i32) -> String {
        let mut id :String = "".to_string();
        // remove vowels from the title
        title.split("").into_iter().for_each(
            |x| {
                if  !matches!(x.to_lowercase().as_str(), "a" | "e" | "i" | "o" | "u"  | " " )  {
                    id.push_str(x);
                }
            }
        );

        let id = id[0..3].to_string() + grade.to_string().as_str() +"0"+ term.to_string().as_str();
        id.to_lowercase()
    }
}

// utils

