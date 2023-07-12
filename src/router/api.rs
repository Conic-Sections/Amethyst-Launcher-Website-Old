#[post("/api/github", format = "application/json", data = "<data>")]
pub async fn github_api(data: String) -> &'static str {
    println!("data: {:?}", data);
    "Ok!"
}
