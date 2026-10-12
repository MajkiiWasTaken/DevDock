/************************************************
* File: gui.rs
* Author: Michal Švrček
*
* Modern graphical session editor for DevDock.
*
* ver. 0.5.1
*************************************************/

use crate::config::{self, CustomCommand, Session, Terminal};
use eframe::egui::{self, Color32, RichText};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

const BG: Color32 = Color32::from_rgb(16, 20, 29);
const SIDEBAR: Color32 = Color32::from_rgb(22, 29, 41);
const PANEL: Color32 = Color32::from_rgb(25, 33, 46);
const BLUE: Color32 = Color32::from_rgb(59, 130, 246);
const WHITE: Color32 = Color32::from_rgb(241, 245, 249);
const MUTED: Color32 = Color32::from_rgb(148, 163, 184);
const GREEN: Color32 = Color32::from_rgb(74, 222, 128);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Page {
    General,
    Vscode,
    Folders,
    Terminals,
    Websites,
    Commands,
}

impl Page {
    fn title(self) -> &'static str {
        match self {
            Self::General => "General",
            Self::Vscode => "VS Code projects",
            Self::Folders => "Explorer folders",
            Self::Terminals => "Terminals",
            Self::Websites => "Websites",
            Self::Commands => "Custom commands",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::General => "Configure your development session.",
            Self::Vscode => "Choose projects to open in Visual Studio Code.",
            Self::Folders => "Select additional folders to open in Explorer.",
            Self::Terminals => "Configure terminal windows and working directories.",
            Self::Websites => "Add websites to open with your workspace.",
            Self::Commands => "Define programs to launch alongside the session.",
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Self::General => "⚙",
            Self::Vscode => "⌘",
            Self::Folders => "▣",
            Self::Terminals => ">_",
            Self::Websites => "◎",
            Self::Commands => "▸",
        }
    }
}

struct Editor {
    session: Session,
    original_name: Option<String>,
    saved_snapshot: String,
    page: Page,
    notice: Option<(String, bool)>,
    new_website: String,
    confirm_close: bool,
    result: Arc<Mutex<Option<Result<(), String>>>>,
}

impl Editor {
    fn new(
        session: Session,
        original_name: Option<String>,
        result: Arc<Mutex<Option<Result<(), String>>>>,
    ) -> Self {
        let saved_snapshot = Self::snapshot(&session);

        Self {
            session,
            original_name,
            saved_snapshot,
            page: Page::General,
            notice: None,
            new_website: String::new(),
            confirm_close: false,
            result,
        }
    }

    fn snapshot(session: &Session) -> String {
        toml::to_string(session).unwrap_or_default()
    }

    fn dirty(&self) -> bool {
        Self::snapshot(&self.session) != self.saved_snapshot
    }

    fn save(&mut self) {
        if self.session.name.trim().is_empty() {
            self.notice = Some(("Session name is required.".into(), false));
            self.page = Page::General;
            return;
        }

        if self
            .session
            .commands
            .iter()
            .any(|c| c.program.trim().is_empty())
        {
            self.notice = Some(("Custom commands must have a program.".into(), false));
            self.page = Page::Commands;
            return;
        }

        match config::save_session(&self.session, self.original_name.as_deref()) {
            Ok(()) => {
                self.saved_snapshot = Self::snapshot(&self.session);
                self.original_name = Some(self.session.name.clone());
                self.notice = Some(("Session saved successfully.".into(), true));

                if let Ok(mut result) = self.result.lock() {
                    *result = Some(Ok(()));
                }
            }
            Err(error) => {
                self.notice = Some((error, false));
            }
        }
    }

    fn request_close(&mut self, ctx: &egui::Context) {
        if self.dirty() {
            self.confirm_close = true;
        } else {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }

    fn navigation(&mut self, ui: &mut egui::Ui, page: Page, count: Option<usize>) {
        let active = self.page == page;

        let text = format!("{}  {}", page.icon(), page.title());
        let button = egui::Button::new(
            RichText::new(text)
                .color(if active { WHITE } else { MUTED })
                .size(14.0),
        )
        .fill(if active {
            Color32::from_rgb(38, 59, 101)
        } else {
            SIDEBAR
        })
        .min_size(egui::vec2(195.0, 38.0))
        .corner_radius(7);

        if ui.add(button).clicked() {
            self.page = page;
        }

        if let Some(count) = count {
            ui.add_space(2.0);
            ui.label(
                RichText::new(format!("       {count} configured"))
                    .size(11.0)
                    .color(MUTED),
            );
        }

        ui.add_space(6.0);
    }

    fn sidebar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("devdock_sidebar")
            .exact_width(225.0)
            .resizable(false)
            .frame(
                egui::Frame::new()
                    .fill(SIDEBAR)
                    .inner_margin(egui::Margin::same(16)),
            )
            .show(ctx, |ui| {
                ui.add_space(8.0);

                ui.heading(RichText::new("◆ DevDock").color(WHITE).size(23.0).strong());

                ui.label(RichText::new("WORKSPACE MANAGER").color(MUTED).size(10.0));

                ui.add_space(30.0);

                ui.label(RichText::new("SESSION SETTINGS").color(MUTED).size(10.0));

                ui.add_space(12.0);

                self.navigation(ui, Page::General, None);
                self.navigation(ui, Page::Vscode, Some(self.session.vscode.len()));
                self.navigation(ui, Page::Folders, Some(self.session.folders.len()));
                self.navigation(ui, Page::Terminals, Some(self.session.terminals.len()));
                self.navigation(ui, Page::Websites, Some(self.session.websites.len()));
                self.navigation(ui, Page::Commands, Some(self.session.commands.len()));

                ui.add_space(25.0);
                ui.separator();

                ui.label(RichText::new("DevDock v0.5.1").color(MUTED).size(11.0));
            });
    }

    fn path_picker(ui: &mut egui::Ui, label: &str, path: &mut PathBuf) {
        ui.label(RichText::new(label).color(MUTED).size(12.0));
        ui.add_space(5.0);

        ui.horizontal(|ui| {
            let mut value = path.to_string_lossy().to_string();

            let width = (ui.available_width() - 105.0).max(120.0);

            if ui
                .add(egui::TextEdit::singleline(&mut value).desired_width(width))
                .changed()
            {
                *path = PathBuf::from(value);
            }

            if ui.button("Browse...").clicked() {
                let mut dialog = rfd::FileDialog::new();

                if path.is_dir() {
                    dialog = dialog.set_directory(path.as_path());
                }

                if let Some(selected) = dialog.pick_folder() {
                    *path = selected;
                }
            }
        });
    }

    fn directory_list(ui: &mut egui::Ui, label: &str, paths: &mut Vec<PathBuf>) {
        ui.horizontal(|ui| {
            ui.label(RichText::new(label).color(MUTED));

            if ui.button("+ Add folder").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    if !paths.contains(&path) {
                        paths.push(path);
                    }
                }
            }
        });

        ui.add_space(12.0);

        if paths.is_empty() {
            ui.label(
                RichText::new("No folders added yet.")
                    .color(MUTED)
                    .italics(),
            );
        }

        let mut remove = None;

        for (index, path) in paths.iter_mut().enumerate() {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("Folder {}", index + 1))
                            .color(WHITE)
                            .strong(),
                    );

                    if ui.button("Remove").clicked() {
                        remove = Some(index);
                    }
                });

                Self::path_picker(ui, "DIRECTORY", path);
            });

            ui.add_space(9.0);
        }

        if let Some(index) = remove {
            paths.remove(index);
        }
    }

    fn general(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("SESSION NAME").color(MUTED).size(12.0));
        ui.add_space(5.0);

        ui.add_enabled(
            self.original_name.is_none(),
            egui::TextEdit::singleline(&mut self.session.name)
                .desired_width(f32::INFINITY)
                .hint_text("e.g. raspberry-pi"),
        );

        if self.original_name.is_some() {
            ui.label(
                RichText::new("Session names cannot be changed in this version.")
                    .color(MUTED)
                    .size(11.0),
            );
        }

        ui.add_space(24.0);

        Self::path_picker(ui, "MAIN PROJECT DIRECTORY", &mut self.session.folder);

        ui.add_space(24.0);

        egui::Frame::group(ui.style())
            .fill(Color32::from_rgb(17, 26, 41))
            .show(ui, |ui| {
                ui.label(
                    RichText::new("ⓘ  About session configuration")
                        .color(Color32::from_rgb(147, 197, 253))
                        .strong(),
                );

                ui.label(
                    RichText::new(
                        "Your settings are stored in a TOML file. \
                     Existing projects and folders are never modified.",
                    )
                    .color(MUTED),
                );
            });
    }

    fn terminals(&mut self, ui: &mut egui::Ui) {
        if ui.button("+ Add terminal").clicked() {
            self.session.terminals.push(Terminal {
                path: self.session.folder.clone(),
                shell: None,
            });
        }

        ui.add_space(12.0);

        let mut remove = None;

        for (index, item) in self.session.terminals.iter_mut().enumerate() {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("Terminal {}", index + 1))
                            .color(WHITE)
                            .strong(),
                    );

                    if ui.button("Remove").clicked() {
                        remove = Some(index);
                    }
                });

                ui.add_space(8.0);

                Self::path_picker(ui, "WORKING DIRECTORY", &mut item.path);

                ui.add_space(10.0);

                let mut shell = item.shell.clone().unwrap_or_default();

                egui::ComboBox::from_id_salt(("shell", index))
                    .selected_text(if shell.is_empty() {
                        "System default"
                    } else {
                        &shell
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut shell, String::new(), "System default");
                        ui.selectable_value(&mut shell, "powershell".into(), "PowerShell");
                        ui.selectable_value(&mut shell, "cmd".into(), "Command Prompt");
                        ui.selectable_value(&mut shell, "wsl".into(), "WSL");
                        ui.selectable_value(&mut shell, "bash".into(), "Bash");
                    });

                item.shell = if shell.is_empty() { None } else { Some(shell) };
            });

            ui.add_space(10.0);
        }

        if let Some(index) = remove {
            self.session.terminals.remove(index);
        }
    }

    fn websites(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add(
                egui::TextEdit::singleline(&mut self.new_website)
                    .desired_width(420.0)
                    .hint_text("https://example.com"),
            );

            if ui.button("+ Add").clicked() {
                let url = self.new_website.trim();

                if (url.starts_with("http://") || url.starts_with("https://"))
                    && !self.session.websites.iter().any(|s| s == url)
                {
                    self.session.websites.push(url.to_string());
                    self.new_website.clear();
                } else {
                    self.notice = Some(("Enter a unique HTTP or HTTPS URL.".into(), false));
                }
            }
        });

        ui.add_space(14.0);

        let mut remove = None;

        for (index, url) in self.session.websites.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label("◎");
                ui.add(
                    egui::TextEdit::singleline(url)
                        .desired_width((ui.available_width() - 90.0).max(150.0)),
                );

                if ui.button("Remove").clicked() {
                    remove = Some(index);
                }
            });

            ui.add_space(6.0);
        }

        if let Some(index) = remove {
            self.session.websites.remove(index);
        }
    }

    fn commands(&mut self, ui: &mut egui::Ui) {
        if ui.button("+ Add command").clicked() {
            self.session.commands.push(CustomCommand {
                program: String::new(),
                args: Vec::new(),
                cwd: Some(self.session.folder.clone()),
            });
        }

        ui.add_space(12.0);

        let mut remove = None;

        for (index, command) in self.session.commands.iter_mut().enumerate() {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!("Command {}", index + 1))
                            .color(WHITE)
                            .strong(),
                    );

                    if ui.button("Remove").clicked() {
                        remove = Some(index);
                    }
                });

                ui.add_space(8.0);
                ui.label("EXECUTABLE");

                ui.text_edit_singleline(&mut command.program);

                ui.add_space(10.0);
                ui.label("ARGUMENTS");

                let mut remove_arg = None;

                for (arg_index, argument) in command.args.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(argument);

                        if ui.small_button("×").clicked() {
                            remove_arg = Some(arg_index);
                        }
                    });
                }

                if let Some(arg_index) = remove_arg {
                    command.args.remove(arg_index);
                }

                if ui.button("+ Add argument").clicked() {
                    command.args.push(String::new());
                }

                ui.add_space(10.0);

                let cwd = command
                    .cwd
                    .get_or_insert_with(|| self.session.folder.clone());

                Self::path_picker(ui, "WORKING DIRECTORY", cwd);
            });

            ui.add_space(10.0);
        }

        if let Some(index) = remove {
            self.session.commands.remove(index);
        }
    }

    fn main_content(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(BG)
                    .inner_margin(egui::Margin::same(24)),
            )
            .show(ctx, |ui| {
                ui.add_space(4.0);

                ui.heading(RichText::new(self.page.title()).color(WHITE).size(25.0));

                ui.label(
                    RichText::new(self.page.description())
                        .color(MUTED)
                        .size(13.0),
                );

                ui.add_space(22.0);

                egui::ScrollArea::vertical()
                    .id_salt("editor_content")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        egui::Frame::new()
                            .fill(PANEL)
                            .inner_margin(egui::Margin::same(20))
                            .corner_radius(10)
                            .show(ui, |ui| {
                                ui.set_min_width((ui.available_width() - 10.0).max(200.0));

                                match self.page {
                                    Page::General => self.general(ui),
                                    Page::Vscode => Self::directory_list(
                                        ui,
                                        "VS CODE WORKSPACES",
                                        &mut self.session.vscode,
                                    ),
                                    Page::Folders => Self::directory_list(
                                        ui,
                                        "EXPLORER FOLDERS",
                                        &mut self.session.folders,
                                    ),
                                    Page::Terminals => self.terminals(ui),
                                    Page::Websites => self.websites(ui),
                                    Page::Commands => self.commands(ui),
                                }
                            });
                    });
            });
    }

    fn footer(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("devdock_footer")
            .frame(
                egui::Frame::new()
                    .fill(SIDEBAR)
                    .inner_margin(egui::Margin::symmetric(20, 13)),
            )
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if self.dirty() {
                        ui.label(
                            RichText::new("● Unsaved changes")
                                .color(Color32::from_rgb(251, 191, 36)),
                        );
                    } else {
                        ui.label(RichText::new("● All changes saved").color(GREEN));
                    }

                    if let Some((message, success)) = &self.notice {
                        ui.separator();
                        ui.label(RichText::new(message).color(if *success {
                            GREEN
                        } else {
                            Color32::from_rgb(248, 113, 113)
                        }));
                    }

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let save = egui::Button::new(
                            RichText::new("  Save session  ").strong().color(WHITE),
                        )
                        .fill(BLUE)
                        .corner_radius(7);

                        if ui.add(save).clicked() {
                            self.save();
                        }

                        if ui.button("Cancel").clicked() {
                            self.request_close(ctx);
                        }
                    });
                });
            });
    }

    fn close_dialog(&mut self, ctx: &egui::Context) {
        if !self.confirm_close {
            return;
        }

        egui::Window::new("Unsaved changes")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.label("You have unsaved changes.");
                ui.label("Do you want to discard them?");
                ui.add_space(12.0);

                ui.horizontal(|ui| {
                    if ui.button("Keep editing").clicked() {
                        self.confirm_close = false;
                    }

                    if ui.button("Discard changes").clicked() {
                        self.confirm_close = false;
                        self.saved_snapshot = Self::snapshot(&self.session);
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
    }
}

impl eframe::App for Editor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Intercept the window close button when there are unsaved changes.
        if ctx.input(|i| i.viewport().close_requested()) && self.dirty() {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.confirm_close = true;
        }

        self.sidebar(ctx);
        self.footer(ctx);
        self.main_content(ctx);
        self.close_dialog(ctx);
    }
}

pub fn open(name: Option<&str>) -> Result<(), String> {
    let session = if let Some(name) = name {
        config::load(name)?
    } else {
        Session {
            name: String::new(),
            folder: std::env::current_dir().map_err(|e| e.to_string())?,
            vscode: Vec::new(),
            folders: Vec::new(),
            terminals: Vec::new(),
            websites: Vec::new(),
            commands: Vec::new(),
        }
    };

    let result = Arc::new(Mutex::new(None));
    let result_for_app = Arc::clone(&result);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("DevDock – Session Editor")
            .with_inner_size([1050.0, 720.0])
            .with_min_inner_size([800.0, 540.0]),
        ..Default::default()
    };

    eframe::run_native(
        "DevDock Session Editor",
        options,
        Box::new(move |cc| {
            let mut visuals = egui::Visuals::dark();
            visuals.panel_fill = BG;
            visuals.window_fill = PANEL;
            visuals.selection.bg_fill = BLUE;
            cc.egui_ctx.set_visuals(visuals);

            Ok(Box::new(Editor::new(
                session,
                name.map(str::to_owned),
                result_for_app,
            )))
        }),
    )
    .map_err(|e| format!("Cannot open editor: {e}"))?;

    if let Ok(state) = result.lock() {
        if let Some(Err(error)) = state.as_ref() {
            return Err(error.clone());
        }
    }

    Ok(())
}
