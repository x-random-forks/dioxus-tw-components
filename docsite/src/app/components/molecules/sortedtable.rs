use dioxus::prelude::*;
use dioxus_components::molecules::table::*;

pub struct TestState<T: ToTableData> {
    data: Vec<T>,
    sorted_col_index: usize,
    sort_ascending: bool,
}

impl<T: ToTableData> TestState<T> {
    pub fn new(data: Vec<T>) -> Self {
        TestState {
            data,
            sorted_col_index: 0,
            sort_ascending: true,
        }
    }

    pub fn set_sorted_col_index(&mut self, sorted_col_index: usize) {
        self.sorted_col_index = sorted_col_index;
    }

    pub fn get_sorted_col_index(&self) -> usize {
        self.sorted_col_index
    }

    pub fn reverse_data(&mut self) {
        self.data.reverse();
    }

    pub fn toggle_sort_ascending(&mut self) {
        self.sort_ascending = !self.sort_ascending;
    }

    pub fn set_sort_ascending(&mut self, sort_ascending: bool) {
        self.sort_ascending = sort_ascending;
    }

    pub fn get_sort_ascending(&self) -> bool {
        self.sort_ascending
    }
}

#[component]
pub fn SortedTable<T: 'static + std::cmp::PartialEq + ToTableData>(
    data: Vec<T>,
    children: Element,
) -> Element {
    let mut state = use_context_provider(|| Signal::new(TestState::<T>::new(data)));

    rsx!(
        Table {
            TableHeader {
                TableRow {
                    for (index , head) in T::headers_to_strings().iter().enumerate() {
                        TableHead {
                            class: "select-none cursor-pointer space-x-2 relative",
                            onclick: move |_| {
                                let sorted_col_index = state.read().get_sorted_col_index();
                                if sorted_col_index == index {
                                    state.write().reverse_data();
                                    state.write().toggle_sort_ascending();
                                } else {
                                    sort_table_keytype(
                                        &mut state.write().data,
                                        |t: &T| t.to_keytype()[index].clone(),
                                    );
                                    state.write().set_sort_ascending(true);
                                }
                                state.write().set_sorted_col_index(index);
                            },
                            {head}
                            if state.read().get_sorted_col_index() == index {
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 124 124",
                                    width: "12",
                                    height: "12",
                                    class: "fill-foreground inline ml-1 transition-all",
                                    style: if state.read().get_sort_ascending() { "transform: rotate(-180deg)" },
                                    path { d: "M66.18,29.742c-2.301-2.3-6.101-2.3-8.401,0l-56,56c-3.8,3.801-1.1,10.2,4.2,10.2h112c5.3,0,8-6.399,4.2-10.2L66.18,29.742   z" }
                                }
                            }
                        }
                    }
                }
            }
            TableBody {
                for data in state.read().data.iter() {
                    TableRow {
                        for field in data.to_keytype().into_iter() {
                            TableCell { {field.to_string()} }
                        }
                    }
                }
            }
        }
    )
}

// Used to change the sorting type of the data (eg if a field is number we will not sort the same way as string)
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyType {
    String(String),
    Number(i128),
}

impl std::fmt::Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyType::String(str) => {
                write!(f, "{str}")
            }
            KeyType::Number(nb) => {
                write!(f, "{nb}")
            }
        }
    }
}

pub trait ToTableData {
    fn headers_to_strings() -> Vec<&'static str>;
    fn to_keytype(&self) -> Vec<KeyType>;
}

fn sort_table_keytype<T: ToTableData, F>(data: &mut Vec<T>, key_extractor: F)
where
    F: Fn(&T) -> KeyType,
{
    data.sort_by_key(key_extractor);
}

#[component]
pub fn SortedTablePage() -> Element {
    let vec_user = UserTab::get_10_users();

    rsx!(
        "Sorted Table"
        SortedTable { data: vec_user }
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
