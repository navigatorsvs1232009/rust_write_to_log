pub mod formatter;
pub mod writer;

use crate::log::formatter::{format_log};
use crate::log::writer::write_to_file;
use serde::Serialize;
use std::io;

// Общая функция для записи логов
pub async fn write_to_log<T>(data: &T, title: Option<&str>) -> io::Result<()>
where
    T: Serialize + std::fmt::Debug,
{
    // Форматируем данные в строку лога
    let log = format_log(data, title);

    // Записываем строку в файл
    write_to_file(log).await
}