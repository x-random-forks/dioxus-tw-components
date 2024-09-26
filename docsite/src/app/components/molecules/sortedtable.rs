use dioxus::prelude::*;
use dioxus_components::molecules::sortable_table::{KeyType, SortableTable, ToTableData};

#[component]
pub fn SortedTablePage() -> Element {
    let vec_user = UserTab::get_10_users();

    rsx!(
        "Sorted Table"
        SortableTable { data: vec_user }
    )
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
}

impl ToTableData for UserTab {
    fn headers_to_strings() -> Vec<&'static str> {
        vec![
            "login", "status", "exam2", "exam3", "exam4", "exam5", "exam6", "libft", "gnl",
            "printf",
        ]
    }

    fn to_keytype(&self) -> Vec<KeyType> {
        vec![
            KeyType::String(self.login.to_string()),
            KeyType::String(self.status.to_string()),
            KeyType::Number(self.exam2.into()),
            KeyType::Number(self.exam3.into()),
            KeyType::Number(self.exam4.into()),
            KeyType::Number(self.exam5.into()),
            KeyType::Number(self.exam6.into()),
            KeyType::Number(self.libft.into()),
            KeyType::Number(self.gnl.into()),
            KeyType::Number(self.printf.into()),
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
            },
        ]
    }
}
