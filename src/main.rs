use eframe::egui::{self, FontData, FontDefinitions, FontFamily, RichText, TextEdit};
use serde::Deserialize;
use std::{collections::HashMap, fs, sync::Arc};

#[derive(Clone, Deserialize)]
struct Sign {
    number: i32,
    symbol: String,
    phonetic: String,
    category: String,
    name_fr: String,
    name_en: String,
}

#[derive(Clone, Copy, PartialEq)]
enum Language { Fr, En }
#[derive(Clone, Copy, PartialEq)]
enum Screen { Select, Keyboard }

struct App {
    screen: Screen,
    language: Language,
    dark: bool,
    signs: Vec<Sign>,
    output: String,
    cursor: usize,
    transliteration: String,
    active_category: String,
    show_numbers: bool,
    message: String,
    lookup: HashMap<String, String>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        install_fonts(&cc.egui_ctx);
        let signs: Vec<Sign> = serde_json::from_str(include_str!("../assets/linear_b.json"))
            .expect("valid embedded Linear B data");
        let lookup = signs.iter().filter(|s| s.phonetic != "??")
            .map(|s| (s.phonetic.trim_end_matches('?').to_lowercase(), s.symbol.clone())).collect();
        Self { screen: Screen::Select, language: Language::Fr, dark: true, signs,
            output: String::new(), cursor: 0, transliteration: String::new(),
            active_category: "main".into(), show_numbers: false, message: String::new(), lookup }
    }
    fn t(&self, fr: &'static str, en: &'static str) -> &'static str { if self.language == Language::Fr { fr } else { en } }
    fn insert(&mut self, text: &str) {
        let idx = self.cursor.min(self.output.chars().count());
        let byte = self.output.char_indices().nth(idx).map(|(i,_)| i).unwrap_or(self.output.len());
        self.output.insert_str(byte, text); self.cursor = idx + text.chars().count();
    }
    fn backspace(&mut self) {
        if self.cursor == 0 { return; }
        let end = self.output.char_indices().nth(self.cursor).map(|(i,_)| i).unwrap_or(self.output.len());
        let start = self.output.char_indices().nth(self.cursor-1).map(|(i,_)| i).unwrap_or(0);
        self.output.replace_range(start..end, ""); self.cursor -= 1;
    }
    fn transliteration_of_output(&self) -> String {
        self.output.chars().map(|c| {
            if c.is_whitespace() { " ".into() }
            else { self.signs.iter().find(|s| s.symbol.starts_with(c)).map(|s| s.phonetic.clone()).unwrap_or_else(|| c.to_string()) }
        }).collect::<Vec<_>>().join("")
    }
    fn add_transliteration(&mut self) {
        let input = self.transliteration.trim().to_lowercase();
        if input.is_empty() { return; }
        if let Some(s) = self.lookup.get(&input).cloned() { self.insert(&s); self.transliteration.clear(); self.message.clear(); }
        else { self.message = self.t("Translittération inconnue", "Unknown transliteration").into(); }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.dark { ctx.set_visuals(egui::Visuals::dark()); } else { ctx.set_visuals(egui::Visuals::light()); }
        egui::TopBottomPanel::top("top").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Linear B Keyboard");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(if self.dark { "☀" } else { "☾" }).clicked() { self.dark = !self.dark; }
                    if ui.button(if self.language == Language::Fr { "EN" } else { "FR" }).clicked() {
                        self.language = if self.language == Language::Fr { Language::En } else { Language::Fr };
                    }
                    if self.screen == Screen::Keyboard && ui.button(self.t("← Langues", "← Languages")).clicked() { self.screen = Screen::Select; }
                });
            });
        });
        match self.screen {
            Screen::Select => egui::CentralPanel::default().show(ctx, |ui| {
                ui.vertical_centered(|ui| { ui.add_space(100.0); ui.heading(self.t("Choisissez un système d’écriture", "Choose a writing system")); ui.add_space(25.0);
                    if ui.add_sized([360.0,80.0], egui::Button::new(RichText::new(self.t("Grec mycénien, écriture linéaire B", "Mycenaean Greek, Linear B script")).size(20.0))).clicked() { self.screen=Screen::Keyboard; }
                });
            }),
            Screen::Keyboard => self.keyboard_ui(ctx),
        }
    }
}

impl App {
    fn keyboard_ui(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(RichText::new(self.t("Résultat", "Result")).strong());
            let out = TextEdit::singleline(&mut self.output).font(egui::TextStyle::Heading).desired_width(f32::INFINITY).show(ui);
            if let Some(range) = out.cursor_range { self.cursor = range.primary.index; }
            ui.label(format!("{}: {}", self.t("Translittération", "Transliteration"), self.transliteration_of_output()));
            ui.horizontal(|ui| {
                if ui.button(self.t("⌫ Retour", "⌫ Backspace")).clicked() { self.backspace(); }
                if ui.button(self.t("Tout effacer", "Clear all")).clicked() { self.output.clear(); self.cursor=0; }
                if ui.button(self.t("Copier", "Copy")).clicked() { ctx.copy_text(self.output.clone()); }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(self.t("Saisie translittérée :", "Transliteration input:"));
                let r=ui.add(TextEdit::singleline(&mut self.transliteration).desired_width(150.0).hint_text("ko, a, dwe…"));
                let enter=r.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                if ui.button(self.t("Insérer", "Insert")).clicked() || enter { self.add_transliteration(); }
                if !self.message.is_empty() { ui.colored_label(egui::Color32::LIGHT_RED, &self.message); }
            });
            ui.separator();
            ui.horizontal_wrapped(|ui| {
                for (id,fr,en) in [("main","Syllabes principales","Main syllables"),("special","Syllabes spéciales","Special syllables"),("unknown","Signes indéterminés","Undetermined signs"),("ideogram","Idéogrammes","Ideograms")] {
                    ui.selectable_value(&mut self.active_category,id.into(),self.t(fr,en));
                }
                if ui.button(self.t("Nombres…", "Numbers…")).clicked() { self.show_numbers=true; }
            });
            ui.separator();
            let visible: Vec<Sign> = self.signs.iter().filter(|s| s.category==self.active_category).cloned().collect();
            egui::ScrollArea::vertical().show(ui, |ui| {
                let columns=if self.active_category=="ideogram" { 7 } else { 9 };
                egui::Grid::new("sign_grid").num_columns(columns).spacing([8.0,8.0]).show(ui, |ui| {
                    for (i,s) in visible.iter().enumerate() {
                        let name=if self.language==Language::Fr { &s.name_fr } else { &s.name_en };
                        let body=format!("{}\n{}\n{}",name,s.symbol,s.phonetic);
                        if ui.add_sized([105.0,92.0],egui::Button::new(RichText::new(body).size(16.0))).clicked() { self.insert(&s.symbol); }
                        if (i+1)%columns==0 { ui.end_row(); }
                    }
                });
            });
        });
        let mut open=self.show_numbers;
        egui::Window::new(self.t("Nombres égéens", "Aegean numbers")).open(&mut open).show(ctx, |ui| {
            ui.label(self.t("Valeurs multiplicatives usuelles", "Common multiplicative values"));
            for (cp,value) in [(0x10107,1),(0x10110,10),(0x10119,100),(0x10122,1000),(0x1012B,10000)] {
                let symbol=char::from_u32(cp).unwrap().to_string();
                if ui.add_sized([160.0,45.0],egui::Button::new(format!("{}   {}",symbol,value))).clicked() { self.insert(&symbol); }
            }
        });
        self.show_numbers=open;
    }
}

fn install_fonts(ctx: &egui::Context) {
    let mut fonts=FontDefinitions::default();
    let candidates=["assets/NotoSansLinearB-Regular.ttf", "C:\\Windows\\Fonts\\seguihis.ttf", "/usr/share/fonts/truetype/noto/NotoSansLinearB-Regular.ttf"];
    for path in candidates {
        if let Ok(bytes)=fs::read(path) {
            fonts.font_data.insert("historic".into(),Arc::new(FontData::from_owned(bytes)));
            for family in [FontFamily::Proportional,FontFamily::Monospace] { fonts.families.entry(family).or_default().insert(0,"historic".into()); }
            break;
        }
    }
    ctx.set_fonts(fonts);
}

fn main() -> eframe::Result<()> {
    let options=eframe::NativeOptions { viewport: egui::ViewportBuilder::default().with_inner_size([1120.0,760.0]).with_min_inner_size([800.0,600.0]), ..Default::default() };
    eframe::run_native("Linear B Keyboard",options,Box::new(|cc|Ok(Box::new(App::new(cc)))))
}
