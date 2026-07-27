use crate::config::load_settings;


pub fn calculate_score(text: &str) -> i32 {

    let settings = load_settings();

    let lower = text.to_lowercase();

    let mut score = 0;


    for (word, value) in settings.productive {

        if lower.contains(&word.to_lowercase()) {

            println!(
                "PRODUCTIVE MATCH: {} {}",
                word,
                value
            );

            score += value;

        }

    }


    for (word, value) in settings.distracting {

        if lower.contains(&word.to_lowercase()) {

            println!(
                "DISTRACTION MATCH: {} +{}",
                word,
                value
            );

            score += value;

        }

    }


    score

}
