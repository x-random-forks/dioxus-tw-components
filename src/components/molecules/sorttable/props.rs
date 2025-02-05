use crate::prelude::*;
use dioxus::prelude::*;

pub trait Sortable: ToString + Clonable {
    fn to_sortable(&self) -> KeyType {
        KeyType::String(self.to_string())
    }
}

impl Clone for Box<dyn Sortable> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait Clonable {
    fn clone_box(&self) -> Box<dyn Sortable>;
}

impl<T: Clone + Sortable + 'static> Clonable for T {
    fn clone_box(&self) -> Box<dyn Sortable> {
        Box::new(self.clone())
    }
}

pub trait ToTableData {
    fn headers_to_strings() -> Vec<impl ToString>;
    fn to_keytype(&self) -> Vec<KeyType>;
}

// Used to change the sorting type of the data (eg if a field is number we will not sort the same way as string)
#[derive(Clone)]
pub enum KeyType {
    String(String),
    Integer(i128),
    UnsignedInteger(u128),
    Object(Box<dyn Sortable>),
}

impl From<&str> for KeyType {
    fn from(str: &str) -> Self {
        KeyType::String(str.to_string())
    }
}

impl From<String> for KeyType {
    fn from(str: String) -> Self {
        KeyType::String(str)
    }
}

impl From<i128> for KeyType {
    fn from(nb: i128) -> Self {
        KeyType::Integer(nb)
    }
}

impl From<u128> for KeyType {
    fn from(nb: u128) -> Self {
        KeyType::UnsignedInteger(nb)
    }
}

impl From<i64> for KeyType {
    fn from(nb: i64) -> Self {
        KeyType::Integer(nb.into())
    }
}

impl From<u64> for KeyType {
    fn from(nb: u64) -> Self {
        KeyType::UnsignedInteger(nb.into())
    }
}

impl From<i32> for KeyType {
    fn from(nb: i32) -> Self {
        KeyType::Integer(nb.into())
    }
}

impl From<u32> for KeyType {
    fn from(nb: u32) -> Self {
        KeyType::UnsignedInteger(nb.into())
    }
}

impl From<i16> for KeyType {
    fn from(nb: i16) -> Self {
        KeyType::Integer(nb.into())
    }
}

impl From<u16> for KeyType {
    fn from(nb: u16) -> Self {
        KeyType::UnsignedInteger(nb.into())
    }
}

impl From<i8> for KeyType {
    fn from(nb: i8) -> Self {
        KeyType::Integer(nb.into())
    }
}

impl From<u8> for KeyType {
    fn from(nb: u8) -> Self {
        KeyType::UnsignedInteger(nb.into())
    }
}

impl PartialEq for KeyType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (KeyType::String(a), KeyType::String(b)) => a == b,
            (KeyType::Integer(a), KeyType::Integer(b)) => a == b,
            (KeyType::UnsignedInteger(a), KeyType::UnsignedInteger(b)) => a == b,
            (KeyType::Object(a), KeyType::Object(b)) => a.to_sortable() == b.to_sortable(),
            _ => false,
        }
    }
}

impl Eq for KeyType {}

impl PartialOrd for KeyType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (KeyType::String(a), KeyType::String(b)) => a.partial_cmp(b),
            (KeyType::Integer(a), KeyType::Integer(b)) => a.partial_cmp(b),
            (KeyType::UnsignedInteger(a), KeyType::UnsignedInteger(b)) => a.partial_cmp(b),
            (KeyType::Object(a), KeyType::Object(b)) => {
                a.to_sortable().partial_cmp(&b.to_sortable())
            }
            _ => None,
        }
    }
}

impl Ord for KeyType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (KeyType::String(a), KeyType::String(b)) => a.cmp(b),
            (KeyType::Integer(a), KeyType::Integer(b)) => a.cmp(b),
            (KeyType::UnsignedInteger(a), KeyType::UnsignedInteger(b)) => a.cmp(b),
            (KeyType::Object(a), KeyType::Object(b)) => a.to_sortable().cmp(&b.to_sortable()),
            _ => std::cmp::Ordering::Equal,
        }
    }
}

impl std::fmt::Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyType::String(str) => {
                write!(f, "{str}")
            }
            KeyType::Integer(nb) => {
                write!(f, "{nb}")
            }
            KeyType::UnsignedInteger(nb) => {
                write!(f, "{nb}")
            }
            KeyType::Object(obj) => {
                write!(f, "{}", obj.to_string())
            }
        }
    }
}

#[derive(Clone, PartialEq, Props)]
pub struct SortTableProps<T: 'static + std::clone::Clone + std::cmp::PartialEq + ToTableData> {
    #[props(extends = caption, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    data: Vec<T>,

    children: Element,
}

pub struct SortTableState<T: ToTableData> {
    data: Vec<T>,
    sorted_col_index: usize,
    sort_ascending: bool,
}

impl<T: ToTableData> SortTableState<T> {
    pub fn new(data: Vec<T>) -> Self {
        SortTableState {
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
pub fn SortTable<T: std::clone::Clone + std::cmp::PartialEq + ToTableData>(
    props: SortTableProps<T>,
) -> Element {
    let mut state = use_context_provider(|| Signal::new(SortTableState::<T>::new(props.data)));

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
                        {head.to_string()},
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
