use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/component-css/Home.css");

#[derive(Clone, PartialEq)]
struct Project {
   name: String,
   repository: String,
   desc: String,
   priority: i32,
}

#[component]
pub fn Home() -> Element {
    let projectList: Vec<Project> = vec![ 
    Project {
        name: "name".to_string(),
        repository: "https://github.com/RoyDumblauskas/laptop-config".to_string(),
        desc: "".to_string(),
        priority: 0,
    },
    Project {
        name: "name".to_string(),
        repository: "repository".to_string(),
        desc: "".to_string(),
        priority: 0,
    },
    ];

    rsx! { 
        document::Stylesheet { href: CSS }
        div { id: "home",
            h2 { "Roy's Homelab Page" }
            div { id: "projectList",
                {projectList.iter().map(|project| {
                    rsx! { ProjectElement { project: project.clone() } }
                })}
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
struct ProjectElementPropsDef {
    project: Project,
}

#[component]
fn ProjectElement(project: Project) -> Element {
    rsx! {
        a { id: "projectItem",
        href: "{project.repository}",
        "{project.name}"
        }
    }
}
