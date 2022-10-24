use anyhow::Result;
use regex::Regex;
use scraper::{Html, Selector};
use std::{env, process::Command};

fn main() {
    let word = env::args().into_iter().nth(1).unwrap();
    let word = re_getword(&word).unwrap();
    println!("{}", word);
    get_haici_exp(word.clone());
}

fn re_getword(word: &String) -> Result<String> {
    let re = Regex::new(r"\w+")?
        .captures(word)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str();
    Ok(re.to_string())
}

#[allow(dead_code)]
fn get_clipboad_words() -> Result<String> {
    let output = Command::new("wl-paste").output()?;
    let word = output.stdout;
    let word = String::from_utf8(word)?;
    let re = Regex::new(r"\w+")?
        .captures(&word)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str();
    Ok(re.to_string())
    // Ok(String::from_utf8(word)?)
}

fn get_haici_exp(word: String) {
    let url = format!("http://apii.dict.cn/mini.php?q={}", word);

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    let fragment = Html::parse_fragment(&body);

    //Accessing element attributes
    let selector = Selector::parse(r#"div[id="e"]"#).unwrap();

    let inner_html = fragment.select(&selector).next().unwrap().inner_html();

    for exp in inner_html.split("<br>") {
        println!("{}", exp);
    }
}
