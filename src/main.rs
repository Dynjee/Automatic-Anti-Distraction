mod config;

mod modules {
    pub mod scorer;
    pub mod grayscale;
}

use std::process::Command;
use std::{thread, time};

use modules::scorer::calculate_score;
use modules::grayscale::{
    enable_grayscale,
    disable_grayscale,
};


fn main() {

    loop {

        // Get current active window from Hyprland
        let output = Command::new("hyprctl")
            .args(["activewindow"])
            .output()
            .expect("Failed to run hyprctl");


        let window = String::from_utf8_lossy(&output.stdout);


        // Calculate distraction score
        let score = calculate_score(&window);


        println!("====================");
        println!("{}", window);
        println!("SCORE: {}", score);


        // Decide grayscale or color
        if score >= 10 {

            println!("STATUS: DISTRACTION - GRAYSCALE");

            enable_grayscale();

        } else {

            println!("STATUS: PRODUCTIVE - COLOR");

            disable_grayscale();

        }


        println!("====================");


        // Check every 2 seconds
        thread::sleep(
            time::Duration::from_secs(2)
        );
    }
}
