use chrono::{DateTime, Local};
use serde::Serialize;
use colored::Colorize;
use std::fs::{DirEntry, metadata, read_dir};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use strum::Display;
use tabled::{
    Table, Tabled,
    settings::{
        Alignment, Color, Remove, Style,
        object::{Columns, Rows },
    },
};

#[derive(Debug, Display, Serialize, PartialEq, Eq, PartialOrd, Ord)]
enum FileType {
    Dir,
    File,
}

#[derive(Debug, Tabled, Serialize)]
pub struct EntryFile {
    #[tabled(rename = "Permissions")]
    permissions: String,
    #[tabled(rename = "Modified")]
    modified: String,
    #[tabled(rename = "Size")]
    len_bytes: String,
    #[tabled(skip)]
    f_type: FileType,
    #[tabled(rename = "Name")]
    name: String,
}

impl EntryFile {
    pub fn print_table(path: &PathBuf, all: bool, m_date: bool) {
        let files = EntryFile::get_files(path, all);
        if files.is_empty() {
            println!("{}", "The directory is empty".green())
        } else {
            let mut table = Table::new(files);
            table.with(Style::re_structured_text());
            if !m_date {
                table.with(Remove::column(Columns::one(1)));
                table.modify(Columns::one(1), Alignment::right());
                table.modify(Columns::one(1), Color::FG_BRIGHT_GREEN);
            } else {
                table.modify(Columns::one(2), Alignment::right());
                table.modify(Columns::one(2), Color::FG_BRIGHT_GREEN);
            }
            table.modify(Rows::first(), Color::FG_BLUE);
            table.modify(Rows::first(), Alignment::center());
            println!("{}", table);
        }
    }

    pub fn get_files(path: &Path, all: bool) -> Vec<EntryFile> {
        let mut data = Vec::default();
        if let Ok(read_dir) = read_dir(path) {
            for entry in read_dir {
                if let Ok(file) = entry {
                    if all {
                        EntryFile::map_data(&mut data, file);
                    } else {
                        let temp = file
                            .file_name()
                            .into_string()
                            .unwrap_or("unknown name".into());

                        if temp.chars().next() == Some('.') {
                            continue;
                        } else {
                            EntryFile::map_data(&mut data, file);
                        }
                    }
                }
            }
        }
        data.sort_by(|a, b| a.f_type.cmp(&b.f_type).then_with(|| a.name.cmp(&b.name)));

        data
    }

    fn map_data(data: &mut Vec<EntryFile>, file: DirEntry) {
        if let Ok(meta) = metadata(&file.path()) {
            data.push(EntryFile {
                permissions: EntryFile::color_permissions(&EntryFile::permission_string(meta.mode(), &meta)),
                name: { 

                    let name = file
                    .file_name()
                    .into_string()
                    .unwrap_or("unknown name".into()); 
                    let icon = EntryFile::get_icon(&name, meta.is_dir());
                    let name = format!("{} {}", icon, name);
                    
                    if meta.is_dir() {
                        name.green().to_string()
                    } else {
                        name
                    }
                },
                len_bytes: EntryFile::format_size(meta.len()),
                f_type: if meta.is_file() {
                    FileType::File
                } else {
                    FileType::Dir
                },
                modified: if let Ok(modi) = meta.modified() {
                    let date: DateTime<Local> = modi.into();
                    format!("{}", date.format("%a %b %e %Y %H:%M %p"))
                } else {
                    String::default()
                },
            });
        }
    }

    fn get_icon(name: &str, is_dir: bool) -> &'static str {
        if is_dir {
            return "󰉋";
        }

        match name.rsplit('.').next() {

            Some("rs") => "",
            Some("py") => "",
            Some("js") => "󰌞",
            Some("ts") => "󰛦",
            Some("toml") => "",
            Some("json") => "󰘦",

            Some("md" | "markdown") => "󰍔",
            
            Some("txt") => "󰈙",

            Some("pdf") => "",

            Some("png" | "jpg" | "jpeg" | "webp" | "gif" | "bmp" | "ico") => "󰋩",
            Some("svg") => "󰜡",

            Some("mp4" | "mkv" | "avi" | "mov" | "webm") => "󰕧",

            Some("mp3" | "wav" | "flac" | "ogg" | "m4a") => "󰎆",

            Some("zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar") => "",

            Some("sh" | "bash") => "",
            Some("zsh") => "",
            Some("fish") => "",

            Some("gitignore") => "",
            Some("git") => "",

            Some("conf" | "cfg" | "ini") => "",

            Some("env") => "",

            Some("exe") => "",
            Some("bin") => "",

            Some("ttf" | "otf" | "woff" | "woff2") => "",

            Some("lock") => "󰌾",

            Some("log") => "󰦪",

            _ => "󰈙",
        }
    }

    fn format_size(bytes: u64) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.2} K", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.2} M", bytes as f64 / 1024.0 / 1024.0)
        } else {
            format!("{:.2} G", bytes as f64 / 1024.0 / 1024.0 / 1024.0)
        }
    }



    fn permission_string(mode: u32, metadata: &std::fs::Metadata) -> String {
        let file_type = if metadata.is_dir() { 'd' } else { '-' };

        let permissions = [
            (0o400, 'r'),
            (0o200, 'w'),
            (0o100, 'x'),
            (0o040, 'r'),
            (0o020, 'w'),
            (0o010, 'x'),
            (0o004, 'r'),
            (0o002, 'w'),
            (0o001, 'x'),
        ];

        let mut result = String::with_capacity(10);
        result.push(file_type);

        for (bit, character) in permissions {
            if mode & bit != 0 {
                result.push(character);
            } else {
                result.push('-');
            }
        }

        result
    } 
     
    fn color_permissions(permissions: &str) -> String {
        permissions
            .chars()
            .map(|c| match c {
                'd' => c.to_string().blue().to_string(),
                'r' => c.to_string().green().to_string(),
                'w' => c.to_string().yellow().to_string(),
                'x' => c.to_string().red().to_string(),
                '-' => c.to_string().dimmed().to_string(),
                _ => c.to_string(),
            })
            .collect()
    }

}
