use dioxus::prelude::*;
use itertools::Itertools;

static CSS: Asset = asset!("/assets/component-css/Home.css");
static GH_ICON: Asset = asset!("/assets/github-icon.svg");

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
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
        name: "Laptop Configuration".to_string(),
        repository: "https://github.com/RoyDumblauskas/laptop-config".to_string(),
        desc: "Roy's personal laptop configuration with flakes, home-manager, and sops-nix!".to_string(),
        priority: 2,
    },
    Project {
        name: "Tests Services".to_string(),
        repository: "https://github.com/RoyDumblauskas/tests-service".to_string(),
        desc: "Flake that builds and runs a configurable systemd service to monitor the various other services on Roy's homelab. That's this page!".to_string(),
        priority: 0,
    },
    Project {
        name: "Homelab Configuration".to_string(),
        repository: "https://github.com/RoyDumblauskas/homelab-config".to_string(),
        desc: "".to_string(),
        priority: 1,
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
