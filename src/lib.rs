use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::{Command, Stdio};
use clap::Parser;
use clap_verbosity_flag::{Verbosity, WarnLevel};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct RuhnArgs {
    #[arg(
        long,
        short = 's',
        name = "stories",
        help = "Number of hackernews stories you want to read",
        default_value = "10"
    )]
    stories: usize,

    #[command(flatten)]
    verbose: Verbosity<WarnLevel>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Story {
    pub time: u32,
    pub title: String,
    pub url: String,
    pub by: String,
}

pub async fn run() {
    let args = RuhnArgs::parse();

    let log_level = args
        .verbose
        .log_level()
        .map(|level| level.to_level_filter())
        .unwrap();

    env_logger::Builder::new()
        .format_module_path(log_level == log::LevelFilter::Trace)
        .format_target(log_level == log::LevelFilter::Trace)
        .format_timestamp(Option::None)
        .filter_level(log_level)
        .init();

    let stories = args.stories;
    display(fetch_stories(stories).await)
    
}




pub async fn get_stories_data(num_of_stories: usize) -> Result<Vec<Story>, Error> {
    let mut stories: Vec<Story> = vec![];
    let ids = get_num_of_id().await?;

    for id in &ids[..num_of_stories] {
        let story_url = format!("https://hacker-news.firebaseio.com/v0/item/{}.json", id);

        if let Ok(res) = Client::new().get(story_url).send().await?.json().await {
            stories.push(res);
        };
    }

    Ok(stories)
}



pub async fn get_num_of_id() -> Result<Vec<u32>, Error> {
    let stories_id: Vec<u32> = Client::new()
        .get("https://hacker-news.firebaseio.com/v0/topstories.json")
        .send()
        .await?
        .json()
        .await?;

    Ok(stories_id)
}

pub fn display(text: String) {
    let mut cmd = Command::new("less");
    cmd.stdin(Stdio::piped());

    if let Ok(mut child) = cmd.spawn() {
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes()).unwrap();
        }

        let _status = child.wait().expect("Failed to wait for child process");
        println!();
    } else {
        eprintln!("Failed to execute command");
    }
}




pub async fn fetch_stories(num_stories: usize) -> String {
    let stories = get_stories_data(num_stories).await.unwrap();
    let mut count = 1;
    let mut output = String::new();

    for story in stories.iter() {
        let text = format!(
            "--------
story {}
---------\n
{}
By {}
{}\n\n\n",
            count, story.title, story.by, story.url
        );
        output.push_str(text.as_str());
        count += 1;
    }

    output
}
