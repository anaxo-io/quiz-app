use yew::prelude::*;
use crate::models::Question;

#[derive(Properties, PartialEq)]
pub struct QuestionProps {
    pub question: Question,
    pub selected_option: Option<usize>,
    pub is_submitted: bool,
    pub on_select: Callback<usize>,
}

#[function_component(QuestionComponent)]
pub fn question_component(props: &QuestionProps) -> Html {
    let selected_option = props.selected_option;
    let is_submitted = props.is_submitted;
    let correct_answer = props.question.correct_answer_index;
    
    html! {
        <div class="question-container">
            <h2 class="question-text">{ &props.question.text }</h2>
            <div class="options-container">
                {
                    props.question.options.iter().enumerate().map(|(index, option)| {
                        let is_selected = selected_option == Some(index);
                        let is_correct = index == correct_answer;
                        
                        let class = if is_submitted {
                            if is_correct {
                                "option-button correct"
                            } else if is_selected && !is_correct {
                                "option-button incorrect"
                            } else {
                                "option-button"
                            }
                        } else if is_selected {
                            "option-button selected"
                        } else {
                            "option-button"
                        };
                        
                        let on_click = {
                            let on_select = props.on_select.clone();
                            Callback::from(move |_| {
                                if !is_submitted {
                                    on_select.emit(index);
                                }
                            })
                        };
                        
                        html! {
                            <button 
                                class={class}
                                onclick={on_click}
                            >
                                <span class="option-index">{ format!("{}", ('A' as u8 + index as u8) as char) }</span>
                                <span class="option-text">{ option }</span>
                                {
                                    if is_submitted && is_correct {
                                        html! { <span class="checkmark">{ "✓" }</span> }
                                    } else if is_submitted && is_selected && !is_correct {
                                        html! { <span class="cross">{ "✗" }</span> }
                                    } else {
                                        html! {}
                                    }
                                }
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>
            
            {
                if is_submitted {
                    let feedback_class = if selected_option == Some(correct_answer) {
                        "feedback correct"
                    } else {
                        "feedback incorrect"
                    };
                    
                    let feedback_text = if selected_option == Some(correct_answer) {
                        "Correct! Bonne réponse."
                    } else {
                        "Incorrect. La bonne réponse est indiquée en vert."
                    };
                    
                    html! {
                        <div class={feedback_class}>
                            { feedback_text }
                        </div>
                    }
                } else {
                    html! {}
                }
            }
        </div>
    }
}
