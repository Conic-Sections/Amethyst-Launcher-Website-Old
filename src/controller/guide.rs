use std::path::PathBuf;

use anyhow::{anyhow, Result};
use pulldown_cmark::{html, Parser};
use rocket_dyn_templates::{context, Template};

use crate::DOCUMENT_ROOT;

pub async fn main(path: PathBuf) -> Result<Template> {
    let document_root = DOCUMENT_ROOT.clone();

    let markdown_path = if path.to_str().ok_or(anyhow!(""))? == "" {
        document_root.join("index.md")
    } else {
        let mut path = path.clone();
        path.set_extension("md");
        document_root.join(path)
    };

    let markdown_input = tokio::fs::read_to_string(markdown_path.clone()).await?;
    let html_output = markdown_to_html(&markdown_input).await;

    Ok(Template::render(
        "guide",
        context! { content: html_output, file_name: markdown_path.file_name().ok_or(anyhow!(""))?.to_str().ok_or(anyhow!(""))?, title_text: "1" },
    ))
}

pub async fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
