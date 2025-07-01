use serde::Serialize;
use uuid::Uuid;

// Константа для разделителей логов
pub const LOG_SEPARATOR: &str = "\n------------------------\n";

// Функция для форматирования данных в строку лога
pub fn format_log<T>(data: &T, title: Option<&str>) -> String
where
    T: Serialize + std::fmt::Debug,
{
    // Генерация уникального идентификатора для запроса
    let request_id = Uuid::new_v4().to_string();

    // Формируем строку лога
    let mut log = String::new();
    log.push_str(LOG_SEPARATOR);
    log.push_str(&format!("Request ID: {}\n", request_id));
    log.push_str(&format!("Title: {}\n", title.unwrap_or("DEBUG")));
    log.push_str(&format!("{:#?}\n", data)); // Используем Debug для красивого вывода
    log.push_str(LOG_SEPARATOR);

    log
}