use actix_web::{web, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct SensorData {
    temperature: f64,
    humidity: f64,
    pressure: f64,
}

#[derive(Serialize, Deserialize)]
struct Device {
    id: String,
    name: String,
    data: SensorData,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(
                web::resource("/devices")
                    .route(web::get().to(get_devices))
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn get_devices() -> impl Responder {
    let devices = vec![
        Device {
            id: "device-1".to_string(),
            name: "Living Room Sensor".to_string(),
            data: SensorData {
                temperature: 22.5,
                humidity: 60.2,
                pressure: 1013.25,
            },
        },
        Device {
            id: "device-2".to_string(),
            name: "Kitchen Sensor".to_string(),
            data: SensorData {
                temperature: 23.1,
                humidity: 58.1,
                pressure: 1012.75,
            },
        },
    ];

    web::Json(devices)
}