const UPDATE_COMMAND: &[u8] = include_bytes!("../../update.sh");

#[post("/api/github", data = "<data>")]
pub async fn github_api(data: &str) -> &'static str {
    tokio::fs::write("update.sh", UPDATE_COMMAND).await.unwrap();
    let mut command = tokio::process::Command::new("chmod");
    command.current_dir(".");
    command.arg("+x");
    command.arg("update.sh");
    command.spawn().unwrap().wait().await.unwrap();
    let mut command = tokio::process::Command::new("nice");
    command.current_dir(".");
    command.arg("./update.sh");
    command.spawn().unwrap().wait().await.unwrap();
    "Ok!"
}
