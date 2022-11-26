use macroquad::texture::Texture2D;
use ears::{Music, Sound};

// Includes functions.rs
mod functions;
use functions::*;

// Includes states.rs
mod states;
use states::*;

#[macroquad::main(window_conf)]
async fn main() {
    let manager = AssetsManager::new("assets");

    let font = manager.load_font("font/seguisym.ttf").await;
    
    let mut textures: NamedHashMap<Texture2D> = NamedHashMap::new();
    textures.insert("bg1".to_owned(), manager.load_tex("img/bg.png").await);
    textures.insert("bg2".to_owned(), manager.load_tex("img/bg2.png").await);
    textures.insert("bg3".to_owned(), manager.load_tex("img/bg3.png").await);
        
    let mut musics: NamedHashMap<Music> = NamedHashMap::new();
    musics.insert("music1".to_owned(), manager.load_mus("mus/nk_poltergeist.ogg"));
    musics.insert("music2".to_owned(), manager.load_mus("mus/nk_underground.ogg"));
    musics.insert("music3".to_owned(), manager.load_mus("mus/dirty_paws_sine_wavs.ogg"));
    
    let mut sounds: NamedHashMap<Sound> = NamedHashMap::new();
    sounds.insert("correct".to_owned(), manager.load_sfx("sfx/correct.wav"));
    sounds.insert("wrong".to_owned(), manager.load_sfx("sfx/wrong.wav"));
    sounds.insert("lost".to_owned(), manager.load_sfx("sfx/lost.wav"));
    sounds.insert("win".to_owned(), manager.load_sfx("sfx/win.wav"));

    let mut texts: NamedHashMap<(String, String)> = NamedHashMap::new();
    let file_content = manager.load_string_from_file("data/texts.txt").await;
    let error_message = "An error occured while parsing texts.txt";
    
    for line in file_content.lines() {
        if line.starts_with("//") {
            continue;
        }

        let mut it = line.split("::");
        
        let key = it.next().expect(error_message);
        let fr_text = it.next().expect(error_message);
        let en_text = it.next().expect(error_message);

        texts.insert(key.to_owned(), (fr_text.to_owned(), en_text.to_owned()));
    }
    
    let mut shared_components = SharedComponents::new(
        font,
        textures,
        musics,
        sounds,
        texts
    );
    
    loop {
        match shared_components.state {
            State::LanguageSelection => language_selection(&mut shared_components).await,
            State::Tutorial1 => tutorial1(&mut shared_components).await,
            State::Level1 => level1(&mut shared_components).await,
            State::Tutorial2 => tutorial2(&mut shared_components).await,
            State::Level2 => level2(&mut shared_components).await,
            State::Tutorial3 => tutorial3(&mut shared_components).await,
            State::Level3 => level3(&mut shared_components).await,
            State::Lost => lost(&mut shared_components).await,
            State::Win => win(&mut shared_components).await,
            State::Error => error(&mut shared_components).await,
            State::Quit => break,
        }
    }
}
