use std::process::Command;


pub fn enable_grayscale() {

    Command::new("hyprctl")
        .args([
            "keyword",
            "decoration:screen_shader",
            "/home/rhue/.config/hypr/shaders/grayscale.glsl"
        ])
        .output()
        .expect("Failed enabling grayscale");

}


pub fn disable_grayscale() {

    Command::new("hyprctl")
        .args([
            "keyword",
            "decoration:screen_shader",
            ""
        ])
        .output()
        .expect("Failed disabling grayscale");

}
