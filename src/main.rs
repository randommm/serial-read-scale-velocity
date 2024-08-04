use sqlx::sqlite::SqlitePoolOptions;
use std::io::{self, Read, Write};
use std::sync::mpsc;
use std::time::Duration;
use std::time::SystemTime;

fn main() {
    let (tx, rx) = mpsc::channel();

    // start tokio runtime in new thread
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let database_url = "sqlite://db.sqlite3";
            let db_pool = SqlitePoolOptions::new()
                .max_connections(5)
                .connect(database_url)
                .await
                .map_err(|e| format!("DB connection failed: {}", e))
                .unwrap();
            let now = SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|e| format!("Error: {:?}", e))
                .unwrap()
                .as_millis() as i64;
            let session_id = sqlx::query("INSERT INTO sessions (created_at) VALUES ($1)")
                .bind(now)
                .execute(&db_pool)
                .await
                .unwrap();
            let session_id: i64 = session_id.last_insert_rowid();
            dbg!(session_id);
            loop {
                let (buf, read_at) = rx.recv().unwrap();
                let buf = String::from_utf8(buf).unwrap().replace(['g', ' '], "");
                let value: f64 = buf.parse().unwrap();
                print!("+");
                io::stdout().flush().unwrap();

                sqlx::query(
                    "INSERT INTO readings (session_id, value, read_at) VALUES ($1, $2, $3)",
                )
                .bind(session_id)
                .bind(value)
                .bind(read_at)
                .execute(&db_pool)
                .await
                .unwrap();
            }
        });
    });

    let mut port = serialport::new("/dev/ttyUSB0", 9_600)
        .timeout(Duration::from_secs(600))
        .open()
        .expect("Failed to open port")
        .bytes();

    loop {
        let mut buf = Vec::with_capacity(30);
        for val in port.by_ref() {
            let val = val.unwrap();
            // LF
            if val == 0x0A {
                break;
            }
            // CR
            if val == 0x0D {
                continue;
            }
            buf.push(val);
        }
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| format!("Error: {:?}", e))
            .unwrap()
            .as_millis() as i64;
        tx.send((buf, now)).unwrap();
    }
}
