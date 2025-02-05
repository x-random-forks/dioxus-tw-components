use dioxus::prelude::*;
use dioxus_components::molecules::sorttable::{KeyType, SortTable, Sortable, ToTableData};

#[component]
pub fn SortedTablePage() -> Element {
    let vec_user = UserTab::get_10_users();

    rsx!(
        "Sorted Table"
        SortTable::<UserTab> { data: vec_user }
    )
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct TestUser {
    name: String,
    age: u32,
}
impl Sortable for TestUser {
    fn to_sortable(&self) -> KeyType {
        self.age.into()
    }
}
impl std::fmt::Display for TestUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
struct TestProject {
    slug: String,
    id: u64,
}
impl Sortable for TestProject {
    fn to_sortable(&self) -> KeyType {
        self.id.into()
    }
}
impl std::fmt::Display for TestProject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.slug, self.id)
    }
}

#[derive(PartialEq, Clone, Debug)]
struct UserTab {
    login: &'static str,
    status: &'static str,
    exam2: u64,
    exam3: u64,
    exam4: u64,
    exam5: u64,
    exam6: u64,
    libft: u64,
    gnl: u64,
    printf: u64,
    user: TestUser,
    project: TestProject,
}

impl ToTableData for UserTab {
    fn headers_to_strings() -> Vec<impl ToString> {
        vec![
            "login", "status", "exam2", "exam3", "exam4", "exam5", "exam6", "libft", "gnl",
            "printf", "user", "project",
        ]
    }

    fn to_keytype(&self) -> Vec<KeyType> {
        vec![
            self.login.into(),
            self.status.into(),
            self.exam2.into(),
            self.exam3.into(),
            self.exam4.into(),
            self.exam5.into(),
            self.exam6.into(),
            self.libft.into(),
            self.gnl.into(),
            self.printf.into(),
            KeyType::Object(Box::new(self.user.clone())),
            KeyType::Object(Box::new(self.project.clone())),
        ]
    }
}

impl UserTab {
    fn get_10_users() -> Vec<UserTab> {
        vec![
            UserTab {
                login: "user1",
                status: "active",
                exam2: 85,
                exam3: 90,
                exam4: 88,
                exam5: 92,
                exam6: 89,
                libft: 100,
                gnl: 95,
                printf: 98,
                user: TestUser {
                    name: "John".to_string(),
                    age: 25,
                },
                project: TestProject {
                    slug: "project1".to_string(),
                    id: 1,
                },
            },
            UserTab {
                login: "user2",
                status: "inactive",
                exam2: 78,
                exam3: 82,
                exam4: 79,
                exam5: 85,
                exam6: 80,
                libft: 90,
                gnl: 88,
                printf: 92,
                user: TestUser {
                    name: "Jane".to_string(),
                    age: 23,
                },
                project: TestProject {
                    slug: "project2".to_string(),
                    id: 2,
                },
            },
            UserTab {
                login: "user3",
                status: "active",
                exam2: 95,
                exam3: 98,
                exam4: 93,
                exam5: 97,
                exam6: 94,
                libft: 100,
                gnl: 100,
                printf: 100,
                user: TestUser {
                    name: "Alice".to_string(),
                    age: 27,
                },
                project: TestProject {
                    slug: "project3".to_string(),
                    id: 3,
                },
            },
            UserTab {
                login: "user4",
                status: "active",
                exam2: 88,
                exam3: 91,
                exam4: 89,
                exam5: 93,
                exam6: 90,
                libft: 95,
                gnl: 98,
                printf: 96,
                user: TestUser {
                    name: "Bob".to_string(),
                    age: 26,
                },
                project: TestProject {
                    slug: "project4".to_string(),
                    id: 4,
                },
            },
            UserTab {
                login: "user5",
                status: "inactive",
                exam2: 75,
                exam3: 78,
                exam4: 76,
                exam5: 80,
                exam6: 77,
                libft: 85,
                gnl: 82,
                printf: 88,
                user: TestUser {
                    name: "Eve".to_string(),
                    age: 24,
                },
                project: TestProject {
                    slug: "project5".to_string(),
                    id: 5,
                },
            },
            UserTab {
                login: "user6",
                status: "active",
                exam2: 92,
                exam3: 96,
                exam4: 94,
                exam5: 98,
                exam6: 95,
                libft: 100,
                gnl: 99,
                printf: 100,
                user: TestUser {
                    name: "Charlie".to_string(),
                    age: 28,
                },
                project: TestProject {
                    slug: "project6".to_string(),
                    id: 6,
                },
            },
            UserTab {
                login: "user7",
                status: "active",
                exam2: 89,
                exam3: 93,
                exam4: 90,
                exam5: 95,
                exam6: 91,
                libft: 98,
                gnl: 97,
                printf: 99,
                user: TestUser {
                    name: "David".to_string(),
                    age: 29,
                },
                project: TestProject {
                    slug: "project7".to_string(),
                    id: 7,
                },
            },
            UserTab {
                login: "user8",
                status: "inactive",
                exam2: 72,
                exam3: 75,
                exam4: 73,
                exam5: 77,
                exam6: 74,
                libft: 80,
                gnl: 78,
                printf: 82,
                user: TestUser {
                    name: "Frank".to_string(),
                    age: 30,
                },
                project: TestProject {
                    slug: "project8".to_string(),
                    id: 8,
                },
            },
            UserTab {
                login: "user9",
                status: "active",
                exam2: 97,
                exam3: 100,
                exam4: 98,
                exam5: 100,
                exam6: 99,
                libft: 100,
                gnl: 100,
                printf: 100,
                user: TestUser {
                    name: "Grace".to_string(),
                    age: 31,
                },
                project: TestProject {
                    slug: "project9".to_string(),
                    id: 9,
                },
            },
            UserTab {
                login: "user10",
                status: "active",
                exam2: 87,
                exam3: 90,
                exam4: 88,
                exam5: 92,
                exam6: 89,
                libft: 96,
                gnl: 94,
                printf: 97,
                user: TestUser {
                    name: "Helen".to_string(),
                    age: 32,
                },
                project: TestProject {
                    slug: "project10".to_string(),
                    id: 10,
                },
            },
        ]
    }
}
