use futures::future::join_all;
use reqwest::Client;
use tokio::{spawn, try_join};

/// # Task Spawning
/// -----------------
///
/// Task Spawning is the process of running another task when one task is in
/// sleep mode. This process utilizes the idle cpu to continue running another
/// thread when one thread is in waiting mode.
///
/// The following example illustrates the task spawnning process when one task
/// is set to idle.
/// We have multiple API requests to the server, which might have different
/// response time based on the API endpoint. While one request waits the response,
/// another will start requesting the data again.
///
/// In the following example, the first request will have slower response
/// and the second will have faster one, because of which, second thread will be
/// spawnned while first thread waits for the response
async fn request(message: &str, delay: usize) {
    println!("⛔ Request for: {message}");
    let client = Client::new();
    if let Ok(resp) = client
        .get(format!("https://httpbin.org/delay/{delay}"))
        .send()
        .await
    {
        println!("✅ Response for {message}: {}", resp.text().await.unwrap());
    }
}

#[tokio::main]
async fn main() {
    {
        println!("Spawning");

        let slower = spawn(request("First Request", 5));
        let faster = spawn(request("Second Request", 1));

        let _ = try_join!(slower, faster);
    }
    /*
       Output:
     ----------

    Spawning
    ⛔ Request for: First Request
    ⛔ Request for: Second Request
    ✅ Response for Second Request: {
      "args": {},
      ...
      "origin": "27.34.73.162",
      "url": "https://httpbin.org/delay/1"
    }
    ✅ Response for First Request: {
      "args": {},
      ...
      "origin": "27.34.73.162",
      "url": "https://httpbin.org/delay/5"
    }
    */

    // spawning and joining from iterators
    // here, we can see the similar behavior as of above, but in case of using
    // different keywords and identifiers, we create iterators and join all of
    // the responses at once using `future` trait.
    {
        let tasks = vec![
            ("First Request", 5),
            ("Second Request", 2),
            ("Third Request", 1),
        ];

        let handles = tasks
            .into_iter()
            .map(|(message, delay)| spawn(request(message, delay)))
            .collect::<Vec<_>>();
        join_all(handles).await;
    }
}
