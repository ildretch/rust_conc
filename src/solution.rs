use crate::statement::*;
use async_trait::async_trait;
use std::sync::mpsc;

#[async_trait]
pub trait Solution {
    async fn solve(repositories: Vec<ServerName>) -> Option<Binary>;
}

pub struct Solution0;

#[async_trait]
impl Solution for Solution0 {
    async fn solve(repositories: Vec<ServerName>) -> Option<Binary> {
        let mut handles = vec![];
        handles.reserve(repositories.len());
        let (tx, rx) = mpsc::channel::<Binary>();

        repositories.into_iter().for_each(|repo| {
            let tx = tx.clone();

            handles.push(tokio::spawn(async move {
                loop {
                    match download(repo.clone()).await {
                        Ok(binary) => {
                            let _ = tx.send(binary);
                            break;
                        }
                        Err(e) => {
                            println!("{e}. Retrying...");
                        }
                    }
                }
        }))});

        match rx.recv() {
            Ok(binary) => {
                handles.into_iter().for_each(|handle| {
                    handle.abort();
                });

                Some(binary)
            },
            Err(_) => panic!("No worker could handle the task.")
        }
    }
}
