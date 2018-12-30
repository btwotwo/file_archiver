extern crate zip;

use std::io::{self, stdin, Read, Write, BufReader};
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use zip::write::{ZipWriter, FileOptions};

fn main() {
    let path = get_path();
    process_dirs(&path).unwrap();

    println!("Готово.")
}

fn process_dirs(dir: &Path) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                process_dirs(&path).unwrap_or_else(|err| println!("При обработке {} произошла ошибка {}", path.display(), err));
            } else {
                println!("Архивирую {}", path.display());

                archive(&path).unwrap_or_else(|err| println!("При архивации {} возникла ошибка: {}", path.display(), err));

                println!("Удаляю оригинальный файл");

                fs::remove_file(path).unwrap_or_else(|err| println!("При удалении произошла ошибка: {}", err));

                println!("--------------------------------");
            }
        }
    }

    Ok(())
}

fn check_archive(path: &PathBuf) -> io::Result<()> {
    println!("Проверяю {}", path.display());

    let archive_file = File::open(&path)?;
    let reader = BufReader::new(archive_file);
    let arch = zip::ZipArchive::new(reader)?;

    if arch.len() > 0 {
        Ok(())
    } else {
        println!("Ошибка: в архиве отсутствуют файлы.");
        Ok(())
    }

}

fn archive(path: &PathBuf) -> io::Result<()> {
    let zip_path = extend_extension(&path, "zip");

    let zip_file = File::create(&zip_path)?;
    let mut zip = ZipWriter::new(zip_file);

    let method = zip::CompressionMethod::Deflated;
    let options = FileOptions::default().compression_method(method);

    let file_name = path.file_name()
        .unwrap()
        .to_str()
        .unwrap();

    zip.start_file(file_name, options)?;

    let mut file = File::open(&path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    zip.write_all(&buffer)?;
    
    zip.finish()?;

    check_archive(&zip_path)?;

    println!("Архивация и проверка {} прошла успешно", path.display());
    Ok(())
}

fn extend_extension(path: &PathBuf, ext: &str) -> PathBuf {
    let mut new_ext = match path.extension() {
        Some(ext) => format!("{}.", ext.to_string_lossy()),
        None => String::from("")
    };

    new_ext.push_str(ext);

    path.with_extension(new_ext)
}

fn get_path() -> PathBuf {
    let mut input = String::new();

    loop {
        input.clear();
        println!("Введите путь: ");

        stdin().read_line(&mut input).unwrap();
        let trimmed = input.trim();

        let path = Path::new(&trimmed);
        
        let meta = match fs::metadata(path) {
            Ok(m) => m,
            Err(_) => {
                println!("Путь невалиден"); 
                continue; 
            }
        };

        match meta.is_dir() {
            true => break PathBuf::from(path),
            false => println!("Необходимо указать путь к папке")
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extend_extension_with_extension() {
        let path = PathBuf::from("C:\\Something.exe");
        let modified = extend_extension(&path, "zip");

        assert_eq!(modified.to_str().unwrap(), "C:\\Something.exe.zip");
    }

    #[test]
    fn extend_extension_without_extension() {
        let path = PathBuf::from("C:\\Something\\testFile");
        let modified = extend_extension(&path, "zip");

        assert_eq!(modified.to_str().unwrap(), "C:\\Something\\testFile.zip");
    }
}