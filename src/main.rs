use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, Rejection};

mod handler;

type WebResult<T> = std::result::Result<T, Rejection>;

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: usize,
    pub subscribed_topics: Vec<String>,
}

pub type Clients = Arc<Mutex<HashMap<String, Client>>>;

#[tokio::main]
async fn main() {
    let clients: Clients = Clients::default();
    initialize_clients(&clients).await;

    let read_route = warp::path!("read")
        .and(with_clients(clients.clone()))
        .and_then(handler::read_handler);

    println!("Started server at localhost:8080");
    warp::serve(read_route).run(([0, 0, 0, 0], 8080)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

async fn initialize_clients(clients: &Clients) {
    let mut clients_lock = clients.lock().await;
    clients_lock.insert(
        String::from("87-89-34"),
        Client {
            user_id: 1,
            subscribed_topics: vec![String::from("cats"), String::from("dogs")],
        },
    );
    clients_lock.insert(
        String::from("22-38-21"),
        Client {
            user_id: 2,
            subscribed_topics: vec![String::from("cats"), String::from("reptiles")],
        },
    );
    clients_lock.insert(
        String::from("12-67-22"),
        Client {
            user_id: 3,
            subscribed_topics: vec![
                String::from("mice"),
                String::from("birds"),
                String::from("snakes"),
            ],
        },
    );
}
