//Codigo pra ler dados do influxDB usando Rust

use chrono::{DateTime, FixedOffset};
use influxdb2::{Client, models::Query, FromDataPoint};
use std::time::Duration;
use tokio::time::sleep;

#[derive(Debug, FromDataPoint)]
pub struct SensorData {
    board: String,
    _measurement: String,
    _field: String,
    value: f64,
    time: DateTime<FixedOffset>,
}

impl Default for SensorData {
    fn default() -> Self {
        Self {
            board: "".to_string(),
            _measurement: "temp".to_string(),
            _field: "value".to_string(),
            value: 0_f64,
            time: DateTime::<FixedOffset>::MIN_UTC.with_timezone(&FixedOffset::east_opt(0).unwrap()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = "http://192.168.2.173:8086".to_string();
    let org = "datatrust".to_string();
    let token = "I4znJy_IyWbGWLf0usLpODngQA6ft1k9j5BRIC8aHvF9JCOfmq55T5HMNCimutq6x_XlO19Vl6IKvb_szqSIQA==".to_string();
    let client = Client::new(host, org, token);

    loop {
        let qs = format!("from(bucket: \"sensors\")
            |> range(start: -30m)
            |> filter(fn: (r) => r.board == \"ESP32001\")
            |> last()
        ");

        let query = Query::new(qs);
        let res: Vec<SensorData> = client.query::<SensorData>(Some(query)).await?;
        println!("{:?}", res);

        // Espera por 30 segundos antes de fazer a próxima consulta
        sleep(Duration::from_secs(30)).await;
    }
}
