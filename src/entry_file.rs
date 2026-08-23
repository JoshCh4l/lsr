use chrono::{DateTime, Local};
use owo_colors::OwoColorize;
use serde::Serialize;
use std::fs::{DirEntry, metadata, read_dir};
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use strum::Display;
use tabled::{
    Table, Tabled,
    settings::{
        Alignment, Color, Remove, Style,
        object::{Columns, Rows},
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
    #[tabled(rename = "Type")]
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
            table.modify(Columns::last(), Color::FG_CYAN);
            table.modify(Rows::first(), Color::FG_GREEN);
            if !m_date {
                table.with(Remove::column(Columns::one(1)));
                table.modify(Columns::one(1), Alignment::right());
            } else {
                table.modify(Columns::one(2), Alignment::right());
            }
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
                permissions: EntryFile::permission_string(meta.mode(), &meta),
                name: file
                    .file_name()
                    .into_string()
                    .unwrap_or("unknown name".into()),
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
}
