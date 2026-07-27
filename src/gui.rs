#[path = "storage.rs"]
mod storage;

use storage::{
    Settings,
    load_settings,
    save_settings,
};

use eframe::egui;
use std::collections::HashMap;


pub struct FocusShadeApp {

    distracting: HashMap<String, i32>,
    productive: HashMap<String, i32>,

    keyword: String,
    score: i32,

    is_productive: bool,

}



impl Default for FocusShadeApp {

    fn default() -> Self {

        let settings = load_settings();

        Self {

            distracting: settings.distracting,

            productive: settings.productive,

            keyword: String::new(),

            score: 30,

            is_productive: false,

        }

    }

}



impl eframe::App for FocusShadeApp {


    fn ui(

        &mut self,

        ui: &mut egui::Ui,

        _frame: &mut eframe::Frame,

    ) {


        egui::ScrollArea::vertical()

            .auto_shrink([false; 2])

            .show(ui, |ui| {



            ui.heading("Signal to Noise");

            ui.label(
                "Customize productive and distracting keywords"
            );


            ui.separator();



            // =========================
            // DISTRACTIONS
            // =========================


            ui.heading(
                "Distraction Keywords"
            );


            let mut remove =
                None;


            for (word, score)
                in self.distracting.clone()
            {


                ui.horizontal(|ui| {


                    ui.label(
                        format!(
                            "{} (+{})",
                            word,
                            score
                        )
                    );


                    if ui.button(
                        "Delete"
                    )
                    .clicked()
                    {

                        remove =
                            Some(word);

                    }


                });


            }



            if let Some(word)
                = remove
            {

                self.distracting.remove(
                    &word
                );

                self.save_settings();

            }



            ui.separator();



            // =========================
            // PRODUCTIVE
            // =========================


            ui.heading(
                "Productive Keywords"
            );


            let mut remove =
                None;


            for (word, score)
                in self.productive.clone()
            {


                ui.horizontal(|ui| {


                    ui.label(
                        format!(
                            "{} ({})",
                            word,
                            score
                        )
                    );


                    if ui.button(
                        "Delete"
                    )
                    .clicked()
                    {

                        remove =
                            Some(word);

                    }


                });


            }



            if let Some(word)
                = remove
            {

                self.productive.remove(
                    &word
                );


                self.save_settings();

            }



            ui.separator();



            // =========================
            // ADD KEYWORD
            // =========================


            ui.heading(
                "Add Keyword"
            );


            ui.label(
                "Keyword"
            );


            ui.text_edit_singleline(
                &mut self.keyword
            );



            ui.add(
                egui::Slider::new(
                    &mut self.score,
                    1..=100
                )
                .text(
                    "Score"
                )
            );



            ui.horizontal(|ui| {


                ui.radio_value(
                    &mut self.is_productive,
                    false,
                    "Distraction"
                );


                ui.radio_value(
                    &mut self.is_productive,
                    true,
                    "Productive"
                );


            });



            if ui.button(
                "Add"
            )
            .clicked()
            {


                let word =
                    self.keyword
                    .trim()
                    .to_lowercase();



                if !word.is_empty()
                {


                    if self.is_productive {


                        self.productive.insert(

                            word,

                            -self.score

                        );


                    }
                    else {


                        self.distracting.insert(

                            word,

                            self.score

                        );


                    }



                    self.keyword.clear();


                    self.save_settings();


                }


            }



        });


    }


}



impl FocusShadeApp {


    fn save_settings(&self) {


        save_settings(

            &Settings {

                distracting:
                    self.distracting.clone(),


                productive:
                    self.productive.clone(),

            }

        );


    }


}
