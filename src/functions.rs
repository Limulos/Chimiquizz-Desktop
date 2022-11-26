use std::collections::HashMap;
use named_tuple::named_tuple;
use macroquad::{prelude::*, miniquad::conf::Icon};
use ears::{Music, Sound, AudioController};
use ::rand::{Rng, seq::SliceRandom}; // I have to write ::rand to avoid confusion with macroquad::rand

pub struct AssetsManager {
    assets_folder: String,
}

impl AssetsManager {
    pub fn new(path: &str) -> Self {
        set_pc_assets_folder(path);
        AssetsManager {
            assets_folder: path.to_owned(),
        }
    }
    
    pub async fn load_tex(&self, filename: &str) -> Texture2D {
        load_texture(filename)
            .await
            .expect(&format!("Could not load following texture: {filename}"))
    }
    
    pub async fn load_font(&self, filename: &str) -> Font {
        load_ttf_font(filename)
            .await
            .expect(&format!("Could not load following font: {filename}"))
    }
    
    pub fn load_sfx(&self, filename: &str) -> Sound {
        let path = format!("{}/{filename}", self.assets_folder);
        Sound::new(&path) 
            .expect(&format!("Could not load following sound: {filename}"))
    }
    
    pub fn load_mus(&self, filename: &str) -> Music {
        let path = format!("{}/{filename}", self.assets_folder);
        Music::new(&path)
            .expect(&format!("Could not load following music: {filename}"))
    }
    
    pub async fn load_string_from_file(&self, filename: &str) -> String {
        load_string(filename)
            .await
            .expect(&format!("Could not load following music: {filename}"))
    }
}

named_tuple!(
    pub struct IndexValue<T> {
        // I need to make the attributes public to call the getters in other files
        pub index: usize,
        pub value: T,
    }
);

// This statement enables to compare enum State variables
#[derive(PartialEq, Eq)]
pub enum State {
    LanguageSelection,
    Tutorial1,
    Level1,
    Tutorial2,
    Level2,
    Tutorial3,
    Level3,
    Lost,
    Win,
    Quit,
    Error,
}

pub type NamedHashMap<T> = HashMap<String, T>;

/// Struct holding components shared by most of the game states
pub struct SharedComponents {
    pub font: Font,
    textures: NamedHashMap<Texture2D>,
    musics: NamedHashMap<Music>,
    sounds: NamedHashMap<Sound>,
    texts: NamedHashMap<(String, String)>,
    
    pub language: String,
    current_bg_key: String,
    current_music_key: String,
    
    pub state: State,
    pub error_message: String,
}

impl SharedComponents {
    pub fn new(font: Font, textures: NamedHashMap<Texture2D>, musics: NamedHashMap<Music>, sounds: NamedHashMap<Sound>,
        texts: NamedHashMap<(String, String)>) -> Self {
        let mut result = SharedComponents {
            font,
            textures,
            musics,
            sounds,
            texts,
            language: String::with_capacity(2), // it will be either "fr" or "en"
            current_bg_key: "bg1".to_owned(),
            current_music_key: "music1".to_owned(),
            state: State::LanguageSelection,
            error_message: String::new(),
        };

        for music in result.musics.values_mut() {
            music.set_looping(true);
        }

        for sound in result.sounds.values_mut() {
            sound.set_volume(0.5);
        }

        result
    }

    pub fn get_text(&self, key: &str) -> String {
        if self.language == "fr" {
            return self.texts.get(key).unwrap().0.clone();
        }

        self.texts.get(key).unwrap().1.clone()
    }

    pub fn set_background_key(&mut self, key: &str) {
        self.current_bg_key = key.to_owned();
    }

    pub fn set_music_key(&mut self, key: &str) {
        self.current_music_key = key.to_owned();
    }
    
    /// Saves an error message to show it later using error() function
    pub fn set_error_message(&mut self, message: String) {
        self.stop_music();
        self.error_message = message;
        self.state = State::Error;
    }

    pub fn set_language(&mut self, language: String) {
        self.language = language;
        self.state = State::Tutorial1;
    }

    pub fn play_sound(&mut self, key: &str) {
        self.sounds.get_mut(key).unwrap().play();
    }

    fn get_mut_music(&mut self) -> &mut Music {
        self.musics.get_mut(&self.current_music_key).unwrap()
    }

    pub fn is_music_playing(&self) -> bool {
        self.musics.get(&self.current_music_key).unwrap().is_playing()
    }

    pub fn restart_music(&mut self) {
        self.get_mut_music().set_offset(0);
    }

    pub fn play_music(&mut self) {
        self.get_mut_music().play();
    }

    pub fn pause_music(&mut self) {
        self.get_mut_music().pause();
    }

    pub fn stop_music(&mut self) {
        self.get_mut_music().stop();
    }

    pub fn draw_background(&self) {
        draw_texture(*self.textures.get(&self.current_bg_key).unwrap(), 0.0, 0.0, WHITE);
    }
}

pub fn window_conf() -> Conf {
    Conf {
       window_title: "Chimiquizz".to_owned(),
       window_width: 640,
       window_height: 360,
       window_resizable: false,
       icon: Some(Icon::miniquad_logo()),
       ..Default::default()
    }
}

fn swap_names(mut english_name: String) -> String {
    english_name = english_name.to_ascii_lowercase();
                
    let v: Vec<&str> = english_name.split(' ').collect();
    english_name = format!("{} {}", v[1], v[0]);
    // Capitalizes the first letter
    english_name[0..1].to_uppercase() + &english_name[1..]
}

fn decode_symbol(symbol: &str, mut english_name: String, filename: &str, line: usize) -> Result<String, String> {
    match symbol {
        "*i" => {
            let last_occurence = english_name.rfind('y').unwrap();
            // This method allows to replace a character from a certain index
            english_name.replace_range(last_occurence..last_occurence+1, "i");
        },
        "*l" => { english_name.remove(english_name.len()-1); },
        "*ium" => { 
            english_name.remove(english_name.len()-1);
            english_name.push_str("ium");
        },
        "*ide" => { english_name = english_name.replace("ure", "ide"); },
        "*rev" => {
            // Cyanate de sodium => Sodium cyanate
            if english_name.contains(" de ") {
                let last_occurence = english_name.rfind("de").unwrap() - 1;
                english_name.replace_range(last_occurence..last_occurence+3, "");   
            }
                        
            english_name = english_name
                .replace("d'", "")
                .replace("argent", "silver");
            english_name = swap_names(english_name);
        },
        "*acid" => {
            // Acide maleique => Maleic acid
            english_name = english_name
                .replace("que", "c")
                .replace("eux", "ous")
                .replace("cide", "cid"); // Acide => Acid
            english_name = swap_names(english_name);
        },
        // If we reach here, the symbol is wrong (* alone has already treated before)
        symbol => return Err(format!("{filename} - line {line} - {symbol} - unknown symbol")),
    }
    Ok(english_name)
}

pub async fn load_data(shared_components: &mut SharedComponents, filename: &str) -> Result<Vec<Vec<String>>, String> {
    const INFOS_PER_LINE: usize = 6;
    
    let file_content = AssetsManager::new("assets").load_string_from_file(filename).await;

    let mut data = Vec::new();
    let mut chemical_name = String::with_capacity(30);
    
    for (i, line) in file_content.lines().enumerate() {
        // Ignores the comments in the file
        if line.starts_with("//") {
            continue;
        }

        let mut infos = Vec::with_capacity(INFOS_PER_LINE);

        for (j, info) in line.split("::").enumerate() {
            if j == 0 && shared_components.language == "en" {
                chemical_name = info.to_owned();
                continue;
            }
            
            if j == 1 && shared_components.language == "fr" {
                continue;
            }

            if info.contains("*") {
                let mut english_name = chemical_name
                    .replace('é', "e")
                    .replace('è', "e")
                    .replace('ï', "i");
                
                if info == "*" {
                    infos.push(english_name);
                    continue;
                }
                
                let number_of_symbols = info.matches('*').count();
                if number_of_symbols == 1 {
                    english_name = decode_symbol(info, english_name, filename, i+1)?;
                } else {
                    for symbol in info.split(':') {
                        english_name = decode_symbol(symbol, english_name, filename, i+1)?;
                    }
                }
                
                infos.push(english_name);
                continue;
            }

            infos.push(info.to_owned());
        }

        // Following the line format of the file, we must have 5 pieces of information per line
        if infos.len() != INFOS_PER_LINE - 1 {
            return Err(format!("{filename} - line {} follows a bad format", i+1));
        }

        data.push(infos);
    }

    Ok(data)
}

pub fn get_question_with_index(data: &Vec<Vec<String>>) -> IndexValue<&String> {
    let rand = ::rand::thread_rng().gen_range(0..data.len());
    IndexValue::new(rand, &data[rand][0])
}

pub fn get_right_answer(data: &Vec<Vec<String>>, question_index: usize) -> &String {
    &data[question_index][1]
}

/// Returns four shuffled answers (including the right one)
pub fn get_answers(data: &Vec<Vec<String>>, question_index: usize) -> IndexValue<Vec<&String>> {
    let right_answer = get_right_answer(data, question_index);
    let mut answers: Vec<&String> = Vec::with_capacity(data[0].len());
    
    for i in 1..data[0].len() {
        answers.push(&data[question_index][i]);
    }

    // We have to shuffle otherwise the right answer will be still at the 0th place
    let mut rng = ::rand::thread_rng();
    answers.shuffle(&mut rng);
    
    let index = answers
        .iter()
        .position(|answer| *answer == right_answer)
        .unwrap();

    IndexValue::new(index, answers)
}

pub fn draw_question(question: &String, font: &Font) {
    const FONT_SIZE: u16 = 40;

    let mut params = TextParams {
        font: *font,
        font_size: FONT_SIZE,
        color: BLACK,
        ..Default::default()
    };

    let dim = measure_text(&question, Some(*font), FONT_SIZE, 1.0); 

    draw_text_ex(
        &question, 
        (screen_width() - dim.width) / 2.0 + 2.0, 
        screen_height() / 4.0 - dim.height / 2.0 + 2.0, 
        params
    );

    params.color = WHITE;

    draw_text_ex(
        &question, 
        (screen_width() - dim.width) / 2.0, 
        screen_height() / 4.0 - dim.height / 2.0, 
        params
    );
}

pub fn draw_title(question: &String, font: &Font) {
    draw_question(question, font);
}

pub fn draw_answer(answer: &String, font: &Font, pos: &Vec2) {
    const FONT_SIZE: u16 = 25;
    const FONT_OFFSET: f32 = FONT_SIZE as f32 / 2.0;

    let mut params = TextParams {
        font: *font,
        font_size: FONT_SIZE,
        color: BLACK,
        ..Default::default()
    };

    let dim = measure_text(&answer, Some(*font), FONT_SIZE, 1.0);

    draw_text_ex(&answer, pos.x - dim.width / 2.0 + 2.0, pos.y - FONT_OFFSET + 2.0, params);
    params.color = WHITE;
    draw_text_ex(&answer, pos.x - dim.width / 2.0, pos.y - FONT_OFFSET, params);
}

pub fn draw_subtext(answer: &String, font: &Font) {
    draw_answer(answer, font, &vec2(screen_width() / 2.0, screen_height() * 0.75));
}

pub fn draw_button(button: &Rect, color: &Color) {
    draw_rectangle(button.x, button.y, button.w, button.h, *color);
}