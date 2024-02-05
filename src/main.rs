// Código para ler dados do InfluxDB usando Rust

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

fn convert_to_brt(datetime: &DateTime<FixedOffset>) -> String {
    let brt_offset = FixedOffset::west(3 * 3600); // UTC-3
    let brt_time = datetime.with_timezone(&brt_offset);
    brt_time.format("%H:%M:%S %d/%m/%Y").to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = "http://192.168.2.173:8086".to_string();
    let org = "datatrust".to_string();
    let token = "I4znJy_IyWbGWLf0usLpODngQA6ft1k9j5BRIC8aHvF9JCOfmq55T5HMNCimutq6x_XlO19Vl6IKvb_szqSIQA==".to_string();
    let client = Client::new(host, org, token);

    let mut last_data: Option<Vec<SensorData>> = None;

    loop {
        let qs = format!("from(bucket: \"sensors\")
            |> range(start: -30m)
            |> filter(fn: (r) => r.board == \"ESP32001\")
            |> last()
        ");

        let query = Query::new(qs);
        let res: Result<Vec<SensorData>, _> = client.query::<SensorData>(Some(query)).await;

        match res {
            Ok(data) => {
                if !data.is_empty() {
                    for item in &data {
                        println!(
                            "Board: {}, Measurement: {}, Field: {}, Value: {}, Time (BRT): {}",
                            item.board,
                            item._measurement,
                            item._field,
                            item.value,
                            convert_to_brt(&item.time)
                        );
                    }
                    last_data = Some(data);
                } else {
                    println!("Nenhum dado encontrado dentro do intervalo.");
                }
            }
            Err(err) => {
                println!("Erro na consulta: {:?}", err);
            }
        }

        // Espera por 30 segundos antes de fazer a próxima consulta
        sleep(Duration::from_secs(30)).await;
    }
}
