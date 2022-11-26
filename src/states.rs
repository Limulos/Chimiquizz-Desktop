use macroquad::prelude::*;
use named_tuple::named_tuple;

use super::functions::*;

named_tuple!(
    struct TextValue<T> {
        text: String,
        value: T,
    }
);

struct Button {
    rect: Rect,
    text: String,
    default_color: Color,
    hovered_color: Color,
    text_size: f32,
}

impl Button {
    fn new(rect: Rect, text: String, default_color: Color, hovered_color: Color, text_size: f32) -> Self {
        Button { rect, text, default_color, hovered_color, text_size }
    }

    fn is_hovered(&self) -> bool {
        let (x, y) = mouse_position();
        self.rect.contains(vec2(x, y))
    }

    fn draw_text(&self, font: &Font) {
        draw_answer(
            &self.text, font, 
            &vec2(self.rect.x + self.rect.w / 2.0, self.rect.y + self.rect.h + self.text_size / 4.0 - 1.0)
        );
    }

    fn draw(&self, font: &Font) {
        draw_button(&self.rect, if self.is_hovered() { &self.hovered_color } else { &self.default_color });
        self.draw_text(font);
    }
    
}

pub async fn language_selection(shared_components: &mut SharedComponents) {
    const BUTTON_SIZE: Vec2 = vec2(128.0, 32.0);
    const BUTTON_TEXT_SIZE: f32 = 25.0;

    let title = "Choose a language".to_owned();

    let left_button = Button::new(
        Rect::new(
            (screen_width() - BUTTON_SIZE.x) / 4.0, (screen_height() - BUTTON_SIZE.y) * 2.0 / 3.0, 
            BUTTON_SIZE.x, BUTTON_SIZE.y
        ),
        "fr".to_owned(),
        Color::from_rgba(220, 0, 0, 255),
        Color::from_rgba(255, 0, 0, 255),
        BUTTON_TEXT_SIZE
    );

    let right_button = Button::new(
        Rect::new(
            (screen_width() - BUTTON_SIZE.x) * 0.75, (screen_height() - BUTTON_SIZE.y) * 2.0 / 3.0, 
            BUTTON_SIZE.x, BUTTON_SIZE.y
        ),
        "en".to_owned(),
        Color::from_rgba(0, 0, 220, 255),
        Color::from_rgba(0, 0, 255, 255),
        BUTTON_TEXT_SIZE
    );

    loop {
        if is_key_pressed(KeyCode::Escape) {
            shared_components.state = State::Quit;
            break;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            if left_button.is_hovered() {
                shared_components.set_language("fr".to_owned());
                break;
            } else if right_button.is_hovered() {
                shared_components.set_language("en".to_owned());
                break;
            }
        }

        shared_components.draw_background();

        draw_title(&title, &shared_components.font);

        left_button.draw(&shared_components.font);
        right_button.draw(&shared_components.font);

        next_frame().await;
    }
}

// Shows instructions/tutorial text
fn show_tutorial_screen(title: &String, instructions: &String, font: &Font) {
    draw_title(title, font);
    draw_subtext(instructions, font);
}

pub async fn tutorial(
    shared_components: &mut SharedComponents, title: String, instructions: String, bg_key: &str, music_key: &str, 
    game_state: State,
) {
    shared_components.set_background_key(bg_key);
    shared_components.set_music_key(music_key);
        
    shared_components.play_music();
    shared_components.restart_music();

    let start_time = get_time();
    
    loop {
        if get_time() - start_time > 2.0 {
            shared_components.state = game_state;
            break;
        }
        
        shared_components.draw_background();
        show_tutorial_screen(&title, &instructions, &shared_components.font);
        
        next_frame().await;
    }
}

pub async fn tutorial1(shared_components: &mut SharedComponents) {
    tutorial(
        shared_components, 
        shared_components.get_text("level1"), 
        shared_components.get_text("subtext1"),
        "bg1",
        "music1", 
        State::Level1
    ).await;
}

pub async fn tutorial2(shared_components: &mut SharedComponents) {
    tutorial(
        shared_components, 
        shared_components.get_text("level2"), 
        shared_components.get_text("subtext2"), 
        "bg2",
        "music2", 
        State::Level2
    ).await;
}

pub async fn tutorial3(shared_components: &mut SharedComponents) {
    tutorial(
        shared_components, 
        shared_components.get_text("level3"), 
        shared_components.get_text("subtext3"),
        "bg3",
        "music3", 
        State::Level3
    ).await;
}

fn get_buttons_pos() -> [Vec2; 4] {
    [
        vec2(screen_width() / 4.0, screen_height() * 3.0 / 4.0),
        vec2(screen_width() * 3.0 / 4.0, screen_height() * 3.0 / 4.0),
        vec2(screen_width() / 4.0, screen_height() * 3.0 / 4.0 + 60.0),
        vec2(screen_width() * 3.0 / 4.0, screen_height() * 3.0 / 4.0 + 60.0)
    ]
}

fn create_buttons() -> [Rect; 4] {
    const BUTTON_SIZE: Vec2 = vec2(275.0, 34.0);

    let buttons_pos = get_buttons_pos();
    let mut buttons = [Rect::new(0.0, 0.0, 0.0, 0.0); 4];

    let it = buttons
        .iter_mut()
        .zip(buttons_pos.iter());
    
    for (button, pos) in it {
        *button = Rect::new(
            pos.x - BUTTON_SIZE.x / 2.0, 
            pos.y - BUTTON_SIZE.y - 2.0, 
            BUTTON_SIZE.x,
            BUTTON_SIZE.y
        );
    }
    
    buttons
}

fn get_colors(value: u8) -> [Color; 4] {
    [
        Color::from_rgba(value, 0, 0, 255),
        Color::from_rgba(0, value, 0, 255),
        Color::from_rgba(0, 0, value, 255),
        Color::from_rgba(value, value, 0, 255),
    ]
}

pub async fn game(shared_components: &mut SharedComponents, filename: &str, music_key: &str, next_tutorial_state: Option<State>) {
    // Score you need to have to go to next level
    const SCORE_TO_PASS: u32 = 20;
    const NUMBER_OF_LIVES: u32 = 3;
    const TIME_LIMIT: f64 = 20.0;

    shared_components.set_music_key(music_key);
    
    let data = load_data(shared_components, filename).await;
    if data.is_err() {
        shared_components.set_error_message(data.unwrap_err());
        return;
    }
    
    let data = data.unwrap();
    
    if data.len() <= SCORE_TO_PASS as usize {
        shared_components.set_error_message(
            format!("Expected {} questions, found {}", SCORE_TO_PASS + NUMBER_OF_LIVES + 1, data.len())
        );
        return;
    }
    
    let mut displayed_questions: Vec<&String> = Vec::with_capacity(data.len());
    
    let mut question = get_question_with_index(&data);
    let mut answers = get_answers(&data, *question.index());
    displayed_questions.push(question.value());

    let lives_format = shared_components.get_text("lives");
    let mut lives = TextValue::new(
        format!("{}: {:02}", lives_format, NUMBER_OF_LIVES), NUMBER_OF_LIVES
    );
    let score_format = shared_components.get_text("score");
    let mut score = TextValue::new(format!("{}: {:02}", score_format, 0u32), 0u32);
    
    let buttons = create_buttons();
    let buttons_pos = get_buttons_pos();
    
    let colors = get_colors(220);
    let hovered_colors = get_colors(255);
    
    let mut start_time = get_time();
    
    let time_format = shared_components.get_text("time");
    let mut time = TextValue::new(format!("{}: {:02.0}", time_format, TIME_LIMIT), TIME_LIMIT);
    
    loop {
        // I put the checks here and not inside the for loop because otherwise it will relaunch the level (even if i break 'main)
        if *score.value() == SCORE_TO_PASS {
            shared_components.stop_music();
            shared_components.state = if let Some(tutorial) = next_tutorial_state {
                tutorial
            } else {
                State::Win
            };
            break;
        } else if *lives.value() == 0 || get_time() - start_time > TIME_LIMIT {
            shared_components.stop_music();
            shared_components.state = State::Lost;
            break;
        }
        
        if is_key_pressed(KeyCode::Escape) {
            shared_components.state = State::Quit;
            break;
        }
        
        if is_key_pressed(KeyCode::Space) {
            if shared_components.is_music_playing() {
                shared_components.pause_music();
            } else {
                shared_components.play_music();
            }
        }

        time.set_value(TIME_LIMIT - (get_time() - start_time));
        time.set_text(format!("{}: {:02.0}", time_format, time.value()));

        shared_components.draw_background();
        draw_question(question.value(), &shared_components.font);

        let it = buttons
            .iter()
            .zip(colors.iter())
            .zip(hovered_colors.iter())
            .enumerate();
        
        for (i, ((button, color), hovered_color)) in it {
            let (x, y) = mouse_position();
            let hovered = button.contains(vec2(x, y));
            
            if hovered {
                if is_mouse_button_pressed(MouseButton::Left) {
                    if i == *answers.index() {
                        score.set_value(score.value() + 1);
                        score.set_text(format!("{}: {:02}", score_format, score.value()));
                        shared_components.play_sound("correct");
                    } else {
                        lives.set_value(lives.value() - 1);
                        lives.set_text(format!("{}: {:02}", lives_format, lives.value()));
                        shared_components.play_sound("wrong");
                    }
                    
                    while displayed_questions.contains(question.value()) {
                        question = get_question_with_index(&data);   
                    }

                    answers = get_answers(&data, *question.index());
                    displayed_questions.push(question.value());

                    start_time = get_time();
                } 
            }

            let color_to_draw = if hovered { hovered_color } else { color };
            draw_button(&button, color_to_draw);
        }

        for (pos, answer) in buttons_pos.into_iter().zip(answers.value().iter()) {
            draw_answer(answer, &shared_components.font, &pos);
        }
        
        draw_answer(&lives.text(), &shared_components.font, &vec2(50.0 + 4.0, 40.0));
        draw_answer(&score.text(), &shared_components.font, &vec2(50.0 + 4.0 * 2.0, 80.0));
        draw_answer(&time.text(), &shared_components.font, &vec2(50.0 + 4.0, 120.0));

        next_frame().await;
    }
}

pub async fn level1(shared_components: &mut SharedComponents) {
    game(shared_components, "data/entities1.lvl", "music1", Some(State::Tutorial2)).await;
}

pub async fn level2(shared_components: &mut SharedComponents) {
    game(shared_components, "data/entities2.lvl", "music2", Some(State::Tutorial3)).await;
}

pub async fn level3(shared_components: &mut SharedComponents) {
    game(shared_components, "data/entities3.lvl", "music3", None).await;
}

pub async fn lost(shared_components: &mut SharedComponents) {
    let mut sound_played_once = false;
        
    let game_over_text = shared_components.get_text("lost");
    let sub_text = shared_components.get_text("lost_subtext");
    
    loop {
        if !sound_played_once {
            shared_components.play_sound("lost");
            sound_played_once = true;
        }
        
        if is_mouse_button_pressed(MouseButton::Left) {
            shared_components.state = State::Tutorial1;
            break;
        }

        if is_key_pressed(KeyCode::Escape) {
            shared_components.state = State::Quit;
            break;
        }
        
        shared_components.draw_background();
    
        draw_title(&game_over_text, &shared_components.font);
        draw_subtext(&sub_text, &shared_components.font);
    
        next_frame().await;
    }
}

pub async fn win(shared_components: &mut SharedComponents) {
    let title = shared_components.get_text("won");

    loop {
        if is_mouse_button_pressed(MouseButton::Left) {
            shared_components.state = State::Tutorial1;
            break;
        }

        if is_key_pressed(KeyCode::Escape) {
            shared_components.state = State::Quit;
            break;
        }
        
        shared_components.draw_background();
        draw_title(&title, &shared_components.font);
        
        next_frame().await;
    }
}

pub async fn error(shared_components: &mut SharedComponents) {
    let title = shared_components.get_text("error");
    
    loop {
        if is_key_pressed(KeyCode::Escape) {
            shared_components.state = State::Quit;
            break;
        }
        
        shared_components.draw_background();

        draw_title(&title, &shared_components.font);
        draw_subtext(&shared_components.error_message, &shared_components.font);
            
        next_frame().await;
    }
}
