use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Question {
    pub text: String,
    pub options: Vec<String>,
    pub correct_answer_index: usize,
}

// Private struct for CSV deserialization
#[derive(Clone, Debug, Deserialize)]
struct QuestionRecord {
    #[serde(default)]
    id: usize,
    question: String,
    option1: String,
    option2: String,
    option3: String,
    option4: String,
    correct_answer_index: usize,
}

impl Question {
    pub fn new(text: &str, options: Vec<&str>, correct_answer_index: usize) -> Self {
        Self {
            text: text.to_string(),
            options: options.iter().map(|s| s.to_string()).collect(),
            correct_answer_index,
        }
    }
    
    // Convert a CSV record to a Question
    fn from_record(record: QuestionRecord) -> Self {
        Self {
            text: record.question,
            options: vec![record.option1, record.option2, record.option3, record.option4],
            correct_answer_index: record.correct_answer_index,
        }
    }
}

// CSV data embedded at compile time
const CSV_DATA: &str = include_str!("../static/questions.csv");

// This function returns all quiz questions loaded from CSV
pub fn get_all_questions() -> Vec<Question> {
    match parse_questions_from_csv() {
        Ok(questions) => questions,
        Err(e) => {
            console_log(&format!("Error loading questions: {}", e));
            get_fallback_questions()
        }
    }
}

// Parse the embedded CSV data
fn parse_questions_from_csv() -> Result<Vec<Question>, csv::Error> {
    let mut reader = csv::Reader::from_reader(CSV_DATA.as_bytes());
    let mut questions = Vec::new();
    
    for result in reader.deserialize() {
        let record: QuestionRecord = result?;
        questions.push(Question::from_record(record));
    }
    
    Ok(questions)
}

// Helper function for logging to console
fn console_log(msg: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(msg));
}

// This function returns a fallback list of questions in case CSV loading fails
fn get_fallback_questions() -> Vec<Question> {
    vec![
        Question::new(
            "Dans quelle période préhistorique les premiers outils en pierre taillée ont-ils été utilisés ?",
            vec![
                "Le Paléolithique inférieur", 
                "Le Néolithique", 
                "Le Mésolithique", 
                "L'âge du bronze"
            ],
            0
        ),
        Question::new(
            "Quel hominidé est associé à la culture moustérienne ?",
            vec![
                "Homo habilis", 
                "Néandertal", 
                "Homo sapiens", 
                "Australopithèque"
            ],
            1
        ),
        Question::new(
            "Comment appelle-t-on les dessins réalisés sur les parois des grottes, comme à Lascaux ?",
            vec![
                "Art rupestre", 
                "Fresque primitive", 
                "Peinture paléolithique", 
                "Pictogramme préhistorique"
            ],
            0
        ),
    ]
}

// Function to get a random selection of N consecutive questions
pub fn get_random_question_sequence(_total_questions: usize, count: usize) -> Vec<Question> {
    use rand::{Rng, SeedableRng};
    use rand::rngs::SmallRng;
    
    let all_questions = get_all_questions();
    
    // Handle the case where we don't have enough questions
    if all_questions.len() <= count {
        return all_questions;
    }
    
    let max_start_index = all_questions.len() - count;
    let mut rng = SmallRng::from_entropy();
    let start_index = rng.gen_range(0..=max_start_index);
    
    all_questions[start_index..(start_index + count)].to_vec()
}
