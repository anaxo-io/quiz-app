use yew::prelude::*;
use models::get_random_question_sequence;
use components::QuestionComponent;

mod models;
mod components;

const QUIZ_SIZE: usize = 5; // Reduced to match our example set
const TOTAL_QUESTIONS: usize = 10; // Set to the number of questions in our example

#[derive(Clone, PartialEq)]
enum QuizState {
    InProgress,
    Completed,
}

#[function_component(App)]
fn app() -> Html {
    let questions = use_state(|| get_random_question_sequence(TOTAL_QUESTIONS, QUIZ_SIZE));
    let current_question = use_state(|| 0);
    let selected_answers = use_state(|| vec![None; QUIZ_SIZE]);
    let is_submitted = use_state(|| false);
    let quiz_state = use_state(|| QuizState::InProgress);
    
    let current_question_index = *current_question;
    let on_option_select = {
        let selected_answers = selected_answers.clone();
        Callback::from(move |option_index: usize| {
            let mut new_answers = (*selected_answers).clone();
            new_answers[current_question_index] = Some(option_index);
            selected_answers.set(new_answers);
        })
    };
    
    let on_submit = {
        let is_submitted = is_submitted.clone();
        Callback::from(move |_| {
            is_submitted.set(true);
        })
    };
    
    let on_next = {
        let current_question = current_question.clone();
        let is_submitted = is_submitted.clone();
        let quiz_state = quiz_state.clone();
        
        Callback::from(move |_| {
            let next_index = *current_question + 1;
            if next_index < QUIZ_SIZE {
                current_question.set(next_index);
                is_submitted.set(false);
            } else {
                quiz_state.set(QuizState::Completed);
            }
        })
    };
    
    let on_retry = {
        let questions = questions.clone();
        let current_question = current_question.clone();
        let selected_answers = selected_answers.clone();
        let is_submitted = is_submitted.clone();
        let quiz_state = quiz_state.clone();
        
        Callback::from(move |_| {
            questions.set(get_random_question_sequence(TOTAL_QUESTIONS, QUIZ_SIZE));
            current_question.set(0);
            selected_answers.set(vec![None; QUIZ_SIZE]);
            is_submitted.set(false);
            quiz_state.set(QuizState::InProgress);
        })
    };
    
    let calculate_score = || {
        let mut score = 0;
        for (i, answer) in (*selected_answers).iter().enumerate() {
            if let Some(selected) = answer {
                if *selected == questions[i].correct_answer_index {
                    score += 1;
                }
            }
        }
        score
    };
    
    html! {
        <div class="app-container">
            <div class="app-header">
                <h1>{ "Quiz de Culture Générale" }</h1>
                <p>{ "Testez vos connaissances avec ces questions variées" }</p>
            </div>
            
            <div class="progress-info">
                <span>{ format!("Question {} sur {}", current_question_index + 1, QUIZ_SIZE) }</span>
                <span>{ format!("{}%", ((current_question_index + 1) as f32 / QUIZ_SIZE as f32 * 100.0) as usize) }</span>
            </div>
            <div class="progress-bar">
                <div class="progress" style={format!("width: {}%", (current_question_index + 1) as f32 / QUIZ_SIZE as f32 * 100.0)}></div>
            </div>
            
            {
                match *quiz_state {
                    QuizState::InProgress => {
                        let q = &questions[current_question_index];
                        let selected = (*selected_answers)[current_question_index];
                        let submitted = *is_submitted;
                        
                        html! {
                            <>
                                <QuestionComponent 
                                    question={q.clone()} 
                                    selected_option={selected}
                                    is_submitted={submitted}
                                    on_select={on_option_select}
                                />
                                
                                <div class="quiz-controls">
                                    {
                                        if !submitted {
                                            html! {
                                                <button 
                                                    class="submit-btn"
                                                    onclick={on_submit}
                                                    disabled={selected.is_none()}
                                                >
                                                    { "Soumettre" }
                                                </button>
                                            }
                                        } else {
                                            html! {
                                                <button 
                                                    class="next-btn"
                                                    onclick={on_next}
                                                >
                                                    { 
                                                        if current_question_index < QUIZ_SIZE - 1 {
                                                            "Question Suivante"
                                                        } else {
                                                            "Voir les Résultats"
                                                        }
                                                    }
                                                </button>
                                            }
                                        }
                                    }
                                </div>
                            </>
                        }
                    },
                    QuizState::Completed => {
                        let score = calculate_score();
                        let percentage = (score as f32 / QUIZ_SIZE as f32 * 100.0) as usize;
                        
                        // Choose emoji based on score
                        let (emoji, message) = match percentage {
                            90..=100 => ("🏆", "Excellent! Vous êtes un expert!"),
                            70..=89 => ("🎉", "Très bien! Vous avez d'excellentes connaissances!"),
                            50..=69 => ("👍", "Bien! Vous avez de bonnes connaissances."),
                            30..=49 => ("🤔", "Pas mal. Continuez à apprendre!"),
                            _ => ("📚", "Continuez à apprendre, vous progressez!"),
                        };
                        
                        html! {
                            <div class="result-container">
                                <h2>{ "Quiz Terminé!" }</h2>
                                <div class="result-emoji">{ emoji }</div>
                                <div class="score">{ score } <span>{ format!("/{}", QUIZ_SIZE) }</span></div>
                                <p>{ format!("Vous avez obtenu {}% de bonnes réponses", percentage) }</p>
                                <p style="margin-top: 1rem; color: var(--neutral-color);">{ message }</p>
                                
                                <button class="retry-btn" onclick={on_retry}>
                                    { "Recommencer" }
                                </button>
                                
                                { if percentage >= 70 {
                                    // Create confetti effect for high scores
                                    (0..20).map(|i| {
                                        let left = format!("{}%", i * 5);
                                        let delay = format!("{}s", (i as f32 * 0.1) % 5.0);
                                        html! {
                                            <div class="confetti" style={format!("left: {}; animation-delay: {}", left, delay)}></div>
                                        }
                                    }).collect::<Html>()
                                } else {
                                    html! {}
                                }}
                            </div>
                        }
                    }
                }
            }
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
