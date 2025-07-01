use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use std::io;

// Асинхронная функция для записи строки в файл
pub async fn write_to_file(log: String) -> io::Result<()> {
    // Асинхронное открытие файла и запись лога
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("hook.log")
        .await?;

    file.write_all(log.as_bytes()).await?;
    Ok(())
}