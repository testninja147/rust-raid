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
fn main() {}
