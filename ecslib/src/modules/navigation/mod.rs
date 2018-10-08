pub mod list;

use crate::templates::ToHtml;
use heck::SnakeCase;

// #[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Insertable, Associations)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[table_name = "projects"]
// #[primary_key(uuid)]
pub struct Link {
    pub visual: String,
    pub url: String,
    pub active: bool,
    pub icon: String,
    pub clearance: Permission,
    pub children: Option<Vec<Link>>,
}
impl Link {
    pub fn new(visual: &str, url: &str) -> Self {
        Link {
            visual: visual.to_string(),
            url: url.to_string(),
            active: false,
            icon: String::new(),
            clearance: Permission::Browse,
            children: None,
        }
    }
    pub fn add_child(&mut self, child: Link) {
        if let Some(chld) = &mut self.children {
            chld.push(child);
        } else {
            let mut chld = Vec::new();
            chld.push(child);
            self.children = Some(chld);
        }
    }
}
impl ToHtml for Link {
    fn to_html(&self, out: &mut dyn std::io::Write) -> std::io::Result<()> {
        let active = if self.active { "active" } else { "" };
        // write!(out, r#"<nav class="nav flex-column">"#)?;
        if let Some(chld) = &self.children {
            let id = self.visual.to_snake_case();
            write!(
                out,
                r##"
            <a class="nav-link text-light {} collapsed" data-target="#{}" data-toggle="collapse" aria-expanded="true" aria-controls="{}" role="button">
            <i class="fa {}"></i>
            <span class="">{}</span>
            <i class="flp fa fa-angle-down fa-pull-right"></i>
            </a>"##,
                active, id, id, self.icon, self.visual
            )?;
            write!(
                out,
                r##"<nav class="nav flex-column collapse" id="{}" data-parent="#togglingnavbar">"##,
                id
            )?;
            for ch in chld {
                ch.to_html(out)?;
            }
            write!(out, r#"</nav>"#)?;
        } else {
            write!(
                out,
                r#"
            <a class="nav-link text-light {}" href="{}">
            <i class="fa {}"></i>
            <span class="">{}</span>
            </a>"#,
                active, self.url, self.icon, self.visual
            )?;
        }
        // write!(out, r#"</nav>"#)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PermissionSet {
    // pub id: i64,
    pub browse: bool,
    pub read: bool,
    pub edit: bool,
    pub add: bool,
    pub delete: bool,
}
impl PermissionSet {
    pub fn as_vec(&self) -> Vec<Permission> {
        let mut res = Vec::new();
        if self.browse {
            res.push(Permission::Browse);
        }
        if self.read {
            res.push(Permission::Read);
        }
        if self.edit {
            res.push(Permission::Edit);
        }
        if self.add {
            res.push(Permission::Add);
        }
        if self.delete {
            res.push(Permission::Delete);
        }
        res
    }
    pub fn deny() -> Self {
        PermissionSet {
            browse: false,
            read: false,
            edit: false,
            add: false,
            delete: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Permission {
    Browse,
    Read,
    Edit,
    Add,
    Delete,
}

pub fn default_menu() -> Vec<Link> {
    vec![
        Link::new("User list", "/user/list"),
        Link::new("My Profile", "/user/"),
    ]
}

pub struct Listing {
    pub id: i64,
    pub name: String,
    pub date: String,
    pub detail_first: String,
    pub detail_last: String,
    pub comment: Option<String>,
    pub edit: Link,
    pub delete: Link,
}
impl Listing {
    pub fn new(name: &str) -> Self {
        Listing::with_id(name, rand::random::<i64>())
    }
    pub fn with_id(name: &str, id: i64) -> Self {
        Listing::with_id_date(name, id, "")
    }
    pub fn with_id_date(name: &str, id: i64, date: &str) -> Self {
        Listing {
            id,
            name: String::from(name),
            date: date.to_owned(),
            detail_first: "".to_owned(),
            detail_last: "".to_owned(),
            comment: None,
            edit: Link::new("", ""),
            delete: Link::new("", ""),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListContext {
    pub head: String,
    pub title: String,
    pub search: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Row {
    pub cells: Vec<Cell>,
    pub links: Vec<Link>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cell {
    pub title: String,
    pub content: CellContent,
    pub is_nullable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CellContent {
    pub title: String,
    pub detail: (String, String, String),
}
impl CellContent {
    pub fn new(title: String) -> Self {
        CellContent {
            title,
            detail: (String::new(), String::new(), String::new()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InputType {
    Input,
    TextArea,
    Select,
    Hidden,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditableField {
    pub input_type: InputType,
    pub title: String,
    pub name: String,
    pub value: String,
    pub links: Vec<Link>,
    pub required: bool,
}
