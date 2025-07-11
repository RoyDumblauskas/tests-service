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
        desc: "Roy's personal laptop configuration with flakes, home-manager, and sops-nix! Not really a homelab service, but it's pretty cool I think.".to_string(),
        priority: 2,
    },
    Project {
        name: "Tests Services".to_string(),
        repository: "https://github.com/RoyDumblauskas/tests-service".to_string(),
        desc: "Flake that builds and runs a configurable systemd service to monitor the various other services on Roy's homelab. That's this page!".to_string(),
        priority: 1,
    },
    Project {
        name: "Homelab Configuration".to_string(),
        repository: "https://github.com/RoyDumblauskas/homelab-config".to_string(),
        desc: "All configuration files for each node in Roy's homelab with flakes, home-manager, and sops-nix. This includes a bootstrap file for the first boot of a machine. The repository also includes the declarations for some homelab services which are not big enough to warrant their own repository (MC & Minio).".to_string(),
        priority: 0,
    },
    ];

    rsx! { 
        document::Stylesheet { href: CSS }
        div { id: "home",
            h2 { "Roy's Homelab Page" }
            div { id: "projectList",
                {projectList.iter().sorted_by(|a, b| Ord::cmp(&a.priority, &b.priority)).map(|project| {
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
            div { id: "link-title",
                img { id: "gh-icon", src: GH_ICON }
                h3 { id: "name", "{project.name}" }
            }
            div { id: "desc",
                "{project.desc}"

            }
        }
    }
}
