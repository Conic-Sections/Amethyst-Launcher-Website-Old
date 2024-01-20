// const UPDATE_COMMAND: &[u8] = include_bytes!("../../update.sh");

#[post("/api/ci", data = "<_data>")]
pub async fn github_ci_api(_data: &str) -> &'static str {
    "Ok!"
    
}

#[post("/api/github", data = "<_data>")]
pub async fn github_api(_data: &str) -> &'static str {
    // tokio::fs::write("update.sh", UPDATE_COMMAND).await.unwrap();
    // let mut command = tokio::process::Command::new("chmod");
    // command.current_dir(".");
    // command.arg("+x");
    // command.arg("update.sh");
    // command.spawn().unwrap().wait().await.unwrap();
    // let mut command = tokio::process::Command::new("nice");
    // command.current_dir(".");
    // command.arg("./update.sh");
    // command.spawn().unwrap().wait().await.unwrap();
    "Ok!"
}
