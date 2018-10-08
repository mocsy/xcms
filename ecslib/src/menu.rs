use crate::modules::navigation::{Link, Permission};

pub fn default_top_menu() -> Vec<Link> {
    let mut lnk = Vec::new();
    // let teams_link = Link {
    //     active: false,
    //     children: None,
    //     clearance: Permission::Browse,
    //     icon: "fa-hourglass-half".to_string(),
    //     url: "/org/select".to_string(),
    //     visual: "Teams".to_string(),
    // };
    // lnk.push(teams_link);

    lnk.push(Link::new("Project list", "/project/list"));
    lnk.push(Link::new("Project todos", "/project/todolist"));

    let myacc_link = Link {
        active: false,
        children: None,
        clearance: Permission::Browse,
        icon: "fa-hourglass-start".to_string(),
        url: "".to_string(),
        visual: "My Account".to_string(),
    };
    lnk.push(myacc_link);

    lnk
}

pub fn default_menu() -> Vec<Link> {
    let mut lnk = Vec::new();

    let mut usrlinks = Vec::new();
    usrlinks.push(Link::new("Users", "/user/list"));
    // usrlinks.push(Link::new("View User*","/user/1"));
    // usrlinks.push(Link::new("Contact information*", "/user/contact"));
    // usrlinks.push(Link::new("Card information*", "/user/card"));
    // let profile_link = Link {
    //     active: false,
    //     children: Some(usrlinks),
    //     clearance: Permission::Browse,
    //     icon: "fa-user-secret".to_string(),
    //     url: "".to_string(),
    //     visual: "My Profile".to_string(),
    // };
    // lnk.push(profile_link);

    // lnk.push(Link::new("Style guide","/"));
    // lnk.push(Link::new("Email builder*","/email/build/0"));

    // let mut org_links = Vec::new();
    // org_links.push(Link::new("Switch", "/org/select"));
    // org_links.push(Link::new("List", "/org/list"));
    // org_links.push(Link::new("Add", "/org/add"));
    // org_links.push(Link::new("Dashboard*", "/org/dashboard"));
    // org_links.push(Link::new("Edit**", "#"));
    // let org_link = Link {
    //     active: false,
    //     children: Some(org_links),
    //     clearance: Permission::Browse,
    //     icon: "fa-gopuram".to_string(),
    //     url: "".to_string(),
    //     visual: "Team".to_string(),
    // };
    // lnk.push(org_link);

    let mut project_links = Vec::new();
    project_links.push(Link::new("List", "/project/list"));
    project_links.push(Link::new("Add", "/project/add"));
    // project_links.push(Link::new("todo*", "/project/todo/0"));
    // project_links.push(Link::new("todo Register*", "/project/register/0"));
    // project_links.push(Link::new("View*", "/project/1"));
    // project_links.push(Link::new("Edit*", "/project/edit/1"));
    let project_link = Link {
        active: false,
        children: Some(project_links),
        clearance: Permission::Browse,
        icon: "fa-ticket-alt".to_string(),
        url: "".to_string(),
        visual: "Project".to_string(),
    };
    lnk.push(project_link);

    lnk
}
