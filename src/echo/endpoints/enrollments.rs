#![allow(dead_code)]

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSection {
    pub section_id: String,
    pub section_name: String,
    pub course_id: String,
    pub course_code: String,
    pub course_name: String,
    pub term_id: String,
    pub lesson_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Term {
    pub id: String,
    pub name: String,
    pub start_date: chrono::NaiveDate,
    pub is_active: bool,
    pub is_active_or_future: bool,
}
type TermsById = HashMap<String, Term>; // Id, term

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Enrollment {
    pub user_sections: Vec<UserSection>,
    #[serde(rename = "termsById")]
    pub terms: TermsById,
}

pub type EnrollmentsRes = [Enrollment; 1]; // just how the API works, enforce 1
