use crate::templates::ToHtml;
use std::collections::HashMap;
use url::Url;

pub fn default_meta(title: &str) -> Meta {
    let mut meta = Meta::new(title);
    if let Ok(fa_css) = Url::parse("https://use.fontawesome.com/releases/v5.7.2/css/all.css") {
        let fa_itg = "sha384-fnmOCqbTlWIlj8LyTjo7mOUStjsKC4pOpQbqyi7RrhN7udi9RwhKkMHpvLbHG9Sr";
        meta.add_external_css(fa_css, fa_itg);
    }
    if let Ok(bs_css) =
        Url::parse("https://stackpath.bootstrapcdn.com/bootstrap/4.2.1/css/bootstrap.min.css")
    {
        let bs_itg = "sha384-GJzZqFGwb1QTTN6wy59ffF1BuGJpLSa9DkKMp0DgiMDm4iYMj70gZWKYbI706tWS";
        meta.add_external_css(bs_css, bs_itg);
    }
    meta.add_local_css("/static/theme.css");

    let jquery = Script::with_external(
        "https://code.jquery.com/jquery-3.3.1.min.js",
        "sha256-FgpCb/KJQlLNfOu91ta32o/NMZxltwRo8QtmkMRdAu8=",
    );
    let popper = Script::with_external(
        "https://cdnjs.cloudflare.com/ajax/libs/popper.js/1.14.6/umd/popper.min.js",
        "sha384-wHAiFfRlMFy6i5SRaxvfOCifBUQy1xHdJ/yoi7FRNXMRBu5WHdZYu1hA6ZOblgut",
    );
    let bootstrap = Script::with_external(
        "https://stackpath.bootstrapcdn.com/bootstrap/4.2.1/js/bootstrap.min.js",
        "sha384-B0UglyR+jN6CkvvICOB2joaf5I4l3gm9GU6Hc1og6Ls7i6U/mkkaduKaBhlAXv9k",
    );
    let theme = Script::new("/static/theme.js");
    meta.add_script(jquery);
    meta.add_script(popper);
    meta.add_script(bootstrap);
    meta.add_script(theme);

    meta
}

pub struct Script {
    pub url: String,
    pub integrity: Option<String>,
    pub crossorigin: Option<String>,
    pub charset: String,
}
impl Script {
    pub fn new(url: &str) -> Self {
        Script {
            url: String::from(url),
            integrity: None,
            crossorigin: None,
            charset: "utf-8".to_owned(),
        }
    }
    pub fn with_external(url: &str, integrity: &str) -> Self {
        Script {
            url: String::from(url),
            integrity: Some(String::from(integrity)),
            crossorigin: Some("anonymous".to_owned()),
            charset: "utf-8".to_owned(),
        }
    }
}
impl Script {
    pub fn as_html(&self) -> String {
        if let Some(itg) = &self.integrity {
            if let Some(cro) = &self.crossorigin {
                format!(
                    r#"<script src="{}" integrity="{}" crossorigin="{}" charset="{}"></script>"#,
                    self.url, itg, cro, self.charset
                )
            } else {
                format!(
                    r#"<script src="{}" integrity="{}" charset="{}"></script>"#,
                    self.url, itg, self.charset
                )
            }
        } else {
            format!(
                r#"<script src="{}" charset="{}"></script>"#,
                self.url, self.charset
            )
        }
    }
}
impl ToHtml for Script {
    fn to_html(&self, out: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(out, "{}", self.as_html())?;
        Ok(())
    }
}

pub struct Meta {
    pub title: String,
    pub stylesheet: HashMap<Url, String>,
    pub local_css: HashMap<String, String>,
    pub viewport: String,
    pub charset: String,
    pub scripts: Vec<Script>,
}
impl Meta {
    pub fn new(title: &str) -> Self {
        Meta {
            title: String::from(title),
            stylesheet: HashMap::new(),
            local_css: HashMap::new(),
            viewport: "width=device-width, initial-scale=1, shrink-to-fit=no".to_owned(),
            charset: "utf-8".to_owned(),
            scripts: Vec::new(),
        }
    }

    pub fn add_local_css(&mut self, url: &str) {
        let lnk = format!(r#"<link rel="stylesheet" href="{}">"#, url);
        self.local_css.insert(String::from(url), lnk);
    }

    pub fn add_external_css(&mut self, url: Url, integrity: &str) {
        let lnk = format!(
            r#"<link rel="stylesheet" href="{}" integrity="{}" crossorigin="anonymous">"#,
            url.as_str(),
            integrity
        );
        self.stylesheet.insert(url, lnk);
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn add_script(&mut self, script: Script) {
        self.scripts.push(script);
    }
}
