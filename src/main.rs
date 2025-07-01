mod log; // Подключаем модуль log

use warp::Filter;
use serde_json::Value;
use log::write_to_log;

#[tokio::main]
async fn main() {
    // Фильтр для обработки POST-запросов
    let log_route = warp::post()
        .and(warp::body::json()) // Извлекаем JSON из тела запроса
        .and_then(|data: Value| async move {
            // Записываем данные в лог
            if let Err(e) = write_to_log(&data, Some("incoming")).await {
                eprintln!("Failed to write to log: {}", e);
                return Err(warp::reject::custom(ServerError::LogError));
            }

            // Возвращаем ответ клиенту
            Ok(warp::reply::with_status(
                "Request logged",
                warp::http::StatusCode::OK,
            ))
        });

    // Обработка ошибок
    let routes = log_route.recover(handle_rejection);

    // Запускаем сервер на порту 3030
    println!("Server started at http://localhost:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Обработка ошибок
#[derive(Debug)]
enum ServerError {
    LogError,
}

impl warp::reject::Reject for ServerError {}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(warp::reply::with_status(
            "Not Found",
            warp::http::StatusCode::NOT_FOUND,
        ))
    } else if let Some(_) = err.find::<ServerError>() {
        Ok(warp::reply::with_status(
            "Internal Server Error: Failed to log request",
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Internal Server Error",
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}