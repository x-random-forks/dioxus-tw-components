use crate::prelude::*;
use dioxus::prelude::*;

pub trait ToTableData {
    fn headers_to_strings() -> Vec<&'static str>;
    fn to_keytype(&self) -> Vec<KeyType>;
}

// Used to change the sorting type of the data (eg if a field is number we will not sort the same way as string)
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeyType {
    String(String),
    Number(isize),
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

#[derive(Clone, PartialEq, Props)]
pub struct SortableTableProps<T: 'static + std::clone::Clone + std::cmp::PartialEq + ToTableData> {
    #[props(extends = caption, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    data: Vec<T>,

    children: Element,
}

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

fn sort_table_keytype<T: ToTableData, F>(data: &mut Vec<T>, key_extractor: F)
where
    F: Fn(&T) -> KeyType,
{
    data.sort_by_key(key_extractor);
}

// TODO Find a way to add the derive UiComp to the component
// Need to edit the dioxus_components_macro crate
pub fn SortableTable<T: 'static + std::clone::Clone + std::cmp::PartialEq + ToTableData>(
    props: SortableTableProps<T>,
) -> Element {
    let mut state = use_context_provider(|| Signal::new(TestState::<T>::new(props.data)));

    rsx!(
        Table {
            TableHeader { TableRow {
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
                        {head},
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
            } }
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
