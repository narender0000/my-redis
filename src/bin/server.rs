use bytes::Bytes;
use mini_redis::Command::{self, Get, Set};
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening on 127.0.0.1:6379");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let db = db.clone();

        println!("Accepted connection");

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) {
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }
}
// std::sync::Mutex (synchronous mutex)
// Blocks the current thread while waiting for the lock.
// This can temporarily stop other async tasks running on that thread.
// It is fine to use in async code if:
// Lock contention is low
// The lock is not held across .await points

// tokio::sync::Mutex (asynchronous mutex)
// When waiting for the lock, the task yields control to the async executor instead of blocking the thread.
// Designed for cases where the lock must be held across .await calls.

// If contention on a synchronous mutex becomes a problem, the best fix is rarely to switch to the Tokio mutex.
// Instead, options to consider are to:
// Let a dedicated task manage state and use message passing.
// Shard the mutex
// Restructure the code to avoid the mutex.
