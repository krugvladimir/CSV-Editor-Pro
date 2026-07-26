// csv_editor_rs.rs — редактор CSV (табличный режим) на Rust

use std::fs::File;
use std::io::{self, Write, BufRead, BufReader, BufWriter};
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;
use termion::{color, style};

struct CSVEditor {
    headers: Vec<String>,
    data: Vec<Vec<String>>,
    filename: Option<String>,
    delimiter: char,
}

impl CSVEditor {
    fn new() -> Self {
        CSVEditor {
            headers: Vec::new(),
            data: Vec::new(),
            filename: None,
            delimiter: ',',
        }
    }

    fn detect_delimiter(&self, line: &str) -> char {
        if line.contains('\t') { '\t' }
        else if line.contains(';') { ';' }
        else { ',' }
    }

    fn split_line(&self, line: &str) -> Vec<String> {
        line.split(self.delimiter).map(|s| s.to_string()).collect()
    }

    fn load(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        let mut first_line = String::new();
        reader.read_line(&mut first_line)?;
        if first_line.is_empty() {
            eprintln!("{}Файл пуст{}", color::Fg(color::Red), style::Reset);
            return Ok(());
        }
        self.delimiter = self.detect_delimiter(&first_line);
        // Переоткрываем файл
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        if let Some(Ok(header_line)) = lines.next() {
            self.headers = self.split_line(&header_line);
        }
        self.data.clear();
        for line in lines {
            if let Ok(l) = line {
                if !l.trim().is_empty() {
                    self.data.push(self.split_line(&l));
                }
            }
        }
        self.filename = Some(filename.to_string());
        println!("{}✅ Загружено {} строк, {} столбцов{}",
                 color::Fg(color::Green), self.data.len(), self.headers.len(), style::Reset);
        Ok(())
    }

    fn save(&self, filename: Option<&str>) -> io::Result<()> {
        let fname = filename.or(self.filename.as_deref()).unwrap_or("");
        if fname.is_empty() {
            eprintln!("{}Нет файла для сохранения{}", color::Fg(color::Red), style::Reset);
            return Ok(());
        }
        let file = File::create(fname)?;
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", self.headers.join(&self.delimiter.to_string()))?;
        for row in &self.data {
            writeln!(writer, "{}", row.join(&self.delimiter.to_string()))?;
        }
        println!("{}✅ Сохранено в {}{}", color::Fg(color::Green), fname, style::Reset);
        Ok(())
    }

    fn list_data(&self, limit: Option<usize>) {
        if self.headers.is_empty() {
            println!("{}Нет данных. Загрузите файл.{}", color::Fg(color::Yellow), style::Reset);
            return;
        }
        let mut col_widths: Vec<usize> = self.headers.iter().map(|h| h.len()).collect();
        let rows_to_show = limit.unwrap_or(self.data.len()).min(self.data.len());
        for i in 0..rows_to_show {
            let row = &self.data[i];
            for j in 0..row.len().min(col_widths.len()) {
                col_widths[j] = col_widths[j].max(row[j].len());
            }
        }
        // Заголовок
        print!("{}  #  {}", color::Fg(color::Cyan), style::Reset);
        for (i, h) in self.headers.iter().enumerate() {
            print!("{:width$} ", h, width = col_widths.get(i).unwrap_or(&0) + 2);
        }
        println!();
        print!("{}-----{}", color::Fg(color::Cyan), style::Reset);
        for w in &col_widths {
            print!("{}", "-".repeat(w + 2));
        }
        println!();
        for i in 0..rows_to_show {
            let row = &self.data[i];
            print!("{:4}  ", i+1);
            for j in 0..self.headers.len() {
                let cell = if j < row.len() { &row[j] } else { "" };
                print!("{:width$} ", cell, width = col_widths.get(j).unwrap_or(&0) + 2);
            }
            println!();
        }
    }

    fn add_row(&mut self, values: Vec<String>) -> bool {
        if values.len() != self.headers.len() {
            eprintln!("{}Ожидается {} значений{}", color::Fg(color::Red), self.headers.len(), style::Reset);
            return false;
        }
        self.data.push(values);
        println!("{}✅ Строка добавлена{}", color::Fg(color::Green), style::Reset);
        true
    }

    fn delete_row(&mut self, row_num: usize) -> bool {
        if row_num < 1 || row_num > self.data.len() {
            eprintln!("{}Неверный номер строки{}", color::Fg(color::Red), style::Reset);
            return false;
        }
        self.data.remove(row_num - 1);
        println!("{}✅ Строка #{} удалена{}", color::Fg(color::Green), row_num, style::Reset);
        true
    }

    fn edit_cell(&mut self, row_num: usize, col_num: usize, value: &str) -> bool {
        if row_num < 1 || row_num > self.data.len() {
            eprintln!("{}Неверный номер строки{}", color::Fg(color::Red), style::Reset);
            return false;
        }
        if col_num < 1 || col_num > self.headers.len() {
            eprintln!("{}Неверный номер столбца{}", color::Fg(color::Red), style::Reset);
            return false;
        }
        let row = &mut self.data[row_num - 1];
        while row.len() < col_num {
            row.push(String::new());
        }
        row[col_num - 1] = value.to_string();
        println!("{}✅ Ячейка обновлена{}", color::Fg(color::Green), style::Reset);
        true
    }

    fn sort_data(&mut self, col_num: usize, reverse: bool) -> bool {
        if col_num < 1 || col_num > self.headers.len() {
            eprintln!("{}Неверный номер столбца{}", color::Fg(color::Red), style::Reset);
            return false;
        }
        let col = col_num - 1;
        self.data.sort_by(|a, b| {
            let va = if col < a.len() { &a[col] } else { "" };
            let vb = if col < b.len() { &b[col] } else { "" };
            if reverse { vb.cmp(va) } else { va.cmp(vb) }
        });
        println!("{}✅ Отсортировано{}", color::Fg(color::Green), style::Reset);
        true
    }

    fn filter_data(&mut self, col_num: usize, value: &str) -> bool {
        if col_num < 1 || col_num > self.headers.len() {
            eprintln!("{}Неверный номер столбца{}", color::Fg(color::Red), style::Reset);
            return false;
        }
        let col = col_num - 1;
        let filtered: Vec<_> = self.data.iter()
            .filter(|row| col < row.len() && row[col] == value)
            .cloned()
            .collect();
        if filtered.is_empty() {
            println!("{}Нет строк, удовлетворяющих фильтру{}", color::Fg(color::Yellow), style::Reset);
        } else {
            self.data = filtered;
            println!("{}✅ Отфильтровано, осталось {} строк{}", color::Fg(color::Green), self.data.len(), style::Reset);
        }
        true
    }

    fn interactive(&mut self) {
        println!("{}📊 CSV Editor Pro — Rust Edition{}", color::Fg(color::Cyan), style::Reset);
        println!("{}Команды: load <file>, list [n], add, delete <row>, edit <row> <col> <value>, sort <col> [desc], filter <col> <value>, save, saveas <file>, exit{}", 
                 color::Fg(color::White), style::Reset);
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut line = String::new();
            if reader.read_line(&mut line).is_err() { break; }
            let line = line.trim();
            if line.is_empty() { continue; }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() { continue; }
            let cmd = parts[0];
            match cmd {
                "exit" => break,
                "load" => {
                    if parts.len() < 2 {
                        eprintln!("{}Укажите имя файла{}", color::Fg(color::Red), style::Reset);
                    } else {
                        if let Err(e) = self.load(parts[1]) {
                            eprintln!("{}Ошибка: {}{}", color::Fg(color::Red), e, style::Reset);
                        }
                    }
                }
                "list" => {
                    let limit = if parts.len() > 1 {
                        parts[1].parse::<usize>().ok()
                    } else { None };
                    self.list_data(limit);
                }
                "add" => {
                    if self.headers.is_empty() {
                        println!("{}Нет данных. Загрузите файл.{}", color::Fg(color::Yellow), style::Reset);
                        continue;
                    }
                    print!("Введите {} значений через запятую: ", self.headers.len());
                    io::stdout().flush().unwrap();
                    let mut vals_line = String::new();
                    reader.read_line(&mut vals_line).unwrap();
                    let vals: Vec<String> = vals_line.trim().split(',').map(|s| s.to_string()).collect();
                    self.add_row(vals);
                }
                "delete" => {
                    if parts.len() < 2 {
                        eprintln!("{}Укажите номер строки{}", color::Fg(color::Red), style::Reset);
                    } else {
                        if let Ok(row) = parts[1].parse::<usize>() {
                            self.delete_row(row);
                        } else {
                            eprintln!("{}Неверный номер{}", color::Fg(color::Red), style::Reset);
                        }
                    }
                }
                "edit" => {
                    if parts.len() < 4 {
                        eprintln!("{}Использование: edit <row> <col> <value>{}", color::Fg(color::Red), style::Reset);
                    } else {
                        if let (Ok(row), Ok(col)) = (parts[1].parse::<usize>(), parts[2].parse::<usize>()) {
                            self.edit_cell(row, col, parts[3]);
                        } else {
                            eprintln!("{}Неверные аргументы{}", color::Fg(color::Red), style::Reset);
                        }
                    }
                }
                "sort" => {
                    if parts.len() < 2 {
                        eprintln!("{}Укажите номер столбца{}", color::Fg(color::Red), style::Reset);
                    } else {
                        if let Ok(col) = parts[1].parse::<usize>() {
                            let reverse = parts.len() > 2 && parts[2] == "desc";
                            self.sort_data(col, reverse);
                        } else {
                            eprintln!("{}Неверный номер{}", color::Fg(color::Red), style::Reset);
                        }
                    }
                }
                "filter" => {
                    if parts.len() < 3 {
                        eprintln!("{}Использование: filter <col> <value>{}", color::Fg(color::Red), style::Reset);
                    } else {
                        if let Ok(col) = parts[1].parse::<usize>() {
                            self.filter_data(col, parts[2]);
                        } else {
                            eprintln!("{}Неверный номер{}", color::Fg(color::Red), style::Reset);
                        }
                    }
                }
                "save" => {
                    if let Err(e) = self.save(None) {
                        eprintln!("{}Ошибка: {}{}", color::Fg(color::Red), e, style::Reset);
                    }
                }
                "saveas" => {
                    if parts.len() < 2 {
                        eprintln!("{}Укажите имя файла{}", color::Fg(color::Red), style::Reset);
                    } else {
                        if let Err(e) = self.save(Some(parts[1])) {
                            eprintln!("{}Ошибка: {}{}", color::Fg(color::Red), e, style::Reset);
                        }
                    }
                }
                _ => eprintln!("{}Неизвестная команда{}", color::Fg(color::Red), style::Reset),
            }
        }
    }
}

fn main() {
    let mut editor = CSVEditor::new();
    editor.interactive();
}
