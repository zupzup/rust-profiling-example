use crate::{Clients, WebResult};
use std::time::Duration;
use warp::{reply, Reply};

pub async fn read_handler(clients: Clients) -> WebResult<impl Reply> {
    let clients_lock = clients.lock().await;
    // let clients_lock = clients.read().await;
    let user_ids: Vec<String> = clients_lock
        .iter()
        .map(|(_, client)| client.user_id.to_string())
        .collect();
    drop(clients_lock);
    tokio::time::sleep(Duration::from_millis(50)).await;
    let result = user_ids
        .iter()
        .rev()
        .map(|user_id| user_id.parse::<usize>().expect("can be parsed to usize"))
        .fold(0, |acc, x| acc + x);
    Ok(reply::html(result.to_string()))
}
