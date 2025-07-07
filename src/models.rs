use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, prelude::*};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

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
    #[allow(dead_code)]
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

// Get the base URL for the application, handles both development and production
fn get_base_url() -> String {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("should have a document on window");
    let location = document.location().expect("document should have a location");
    
    // Get pathname to determine if we're in /quiz-app/ or root
    let pathname = location.pathname().unwrap_or_default();
    
    // If pathname starts with /quiz-app/, we're in production
    if pathname.starts_with("/quiz-app/") {
        return "/quiz-app/".to_string();
    } else {
        return "".to_string();
    }
}

// Load questions from CSV file asynchronously
pub async fn load_questions_from_csv() -> Result<Vec<Question>, JsValue> {
    // Determine base URL and build the path to CSV file
    let base_url = get_base_url();
    let csv_url = format!("{}questions.csv", base_url);
    
    console_log(&format!("Fetching questions from: {}", csv_url));
    
    // Create request
    let opts = RequestInit::new();
    opts.set_method("GET");
    opts.set_mode(RequestMode::Cors);
    
    let request = Request::new_with_str_and_init(&csv_url, &opts)?;
    
    // Fetch request
    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();
    
    if !resp.ok() {
        console_log(&format!("Failed to load CSV: HTTP status {}", resp.status()));
        return Err(JsValue::from_str("Failed to load questions.csv"));
    }
    
    // Get text from response
    let text = JsFuture::from(resp.text()?).await?;
    let csv_text = text.as_string().unwrap();
    
    // Parse CSV
    match parse_csv_string(&csv_text) {
        Ok(questions) => Ok(questions),
        Err(e) => {
            console_log(&format!("Error parsing CSV: {:?}", e));
            Err(JsValue::from_str("Failed to parse questions.csv"))
        }
    }
}

// Parse CSV data from a string
fn parse_csv_string(csv_data: &str) -> Result<Vec<Question>, csv::Error> {
    let mut reader = csv::Reader::from_reader(csv_data.as_bytes());
    let mut questions = Vec::new();
    
    for result in reader.deserialize() {
        let record: QuestionRecord = result?;
        questions.push(Question::from_record(record));
    }
    
    Ok(questions)
}

// Helper function for logging to console
pub fn console_log(msg: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(msg));
}

// This function returns a fallback list of questions in case CSV loading fails
pub fn get_fallback_questions() -> Vec<Question> {
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
        Question::new(
            "Quelle invention marque le début de la sédentarisation au Néolithique ?",
            vec![
                "L'agriculture", 
                "La roue", 
                "L'écriture", 
                "Le feu"
            ],
            0
        ),
        Question::new(
            "Quelle ville grecque antique est connue pour avoir inventé la démocratie ?",
            vec![
                "Sparte", 
                "Athènes", 
                "Corinthe", 
                "Thèbes"
            ],
            1
        ),
    ]
}

// Function to get a random selection of N consecutive questions from loaded questions
pub fn get_random_question_sequence_from_list(questions: &[Question], count: usize) -> Vec<Question> {
    use rand::{Rng, SeedableRng};
    use rand::rngs::SmallRng;
    
    // Handle the case where we don't have enough questions
    if questions.len() <= count {
        return questions.to_vec();
    }
    
    let max_start_index = questions.len() - count;
    let mut rng = SmallRng::from_entropy();
    let start_index = rng.gen_range(0..=max_start_index);
    
    questions[start_index..(start_index + count)].to_vec()
}
