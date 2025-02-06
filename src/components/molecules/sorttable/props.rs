use crate::attributes::*;
use crate::prelude::*;
use dioxus::prelude::*;
use dioxus_components_macro::UiComp;
use tailwind_fuse::tw_merge;

pub trait Sortable: ToString + Clonable {
    fn to_sortable(&self) -> KeyType {
        KeyType::String(self.to_string(), true)
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
    Element(Element, bool),
    String(String, bool),
    Integer(i128, bool),
    UnsignedInteger(u128, bool),
    Object(Box<dyn Sortable>, bool),
}

impl From<&str> for KeyType {
    fn from(str: &str) -> Self {
        KeyType::String(str.to_string(), true)
    }
}

impl From<String> for KeyType {
    fn from(str: String) -> Self {
        KeyType::String(str, true)
    }
}

impl From<i128> for KeyType {
    fn from(nb: i128) -> Self {
        KeyType::Integer(nb, true)
    }
}

impl From<u128> for KeyType {
    fn from(nb: u128) -> Self {
        KeyType::UnsignedInteger(nb, true)
    }
}

impl From<i64> for KeyType {
    fn from(nb: i64) -> Self {
        KeyType::Integer(nb.into(), true)
    }
}

impl From<u64> for KeyType {
    fn from(nb: u64) -> Self {
        KeyType::UnsignedInteger(nb.into(), true)
    }
}

impl From<i32> for KeyType {
    fn from(nb: i32) -> Self {
        KeyType::Integer(nb.into(), true)
    }
}

impl From<u32> for KeyType {
    fn from(nb: u32) -> Self {
        KeyType::UnsignedInteger(nb.into(), true)
    }
}

impl From<i16> for KeyType {
    fn from(nb: i16) -> Self {
        KeyType::Integer(nb.into(), true)
    }
}

impl From<u16> for KeyType {
    fn from(nb: u16) -> Self {
        KeyType::UnsignedInteger(nb.into(), true)
    }
}

impl From<i8> for KeyType {
    fn from(nb: i8) -> Self {
        KeyType::Integer(nb.into(), true)
    }
}

impl From<u8> for KeyType {
    fn from(nb: u8) -> Self {
        KeyType::UnsignedInteger(nb.into(), true)
    }
}

impl PartialEq for KeyType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (KeyType::String(a, true), KeyType::String(b, true)) => a == b,
            (KeyType::Integer(a, true), KeyType::Integer(b, true)) => a == b,
            (KeyType::UnsignedInteger(a, true), KeyType::UnsignedInteger(b, true)) => a == b,
            (KeyType::Object(a, true), KeyType::Object(b, true)) => {
                a.to_sortable() == b.to_sortable()
            }
            _ => false,
        }
    }
}

impl Eq for KeyType {}

impl PartialOrd for KeyType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (KeyType::String(a, true), KeyType::String(b, true)) => a.partial_cmp(b),
            (KeyType::Integer(a, true), KeyType::Integer(b, true)) => a.partial_cmp(b),
            (KeyType::UnsignedInteger(a, true), KeyType::UnsignedInteger(b, true)) => {
                a.partial_cmp(b)
            }
            (KeyType::Object(a, true), KeyType::Object(b, true)) => {
                a.to_sortable().partial_cmp(&b.to_sortable())
            }
            _ => None,
        }
    }
}

impl Ord for KeyType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (KeyType::String(a, true), KeyType::String(b, true)) => a.cmp(b),
            (KeyType::Integer(a, true), KeyType::Integer(b, true)) => a.cmp(b),
            (KeyType::UnsignedInteger(a, true), KeyType::UnsignedInteger(b, true)) => a.cmp(b),
            (KeyType::Object(a, true), KeyType::Object(b, true)) => {
                a.to_sortable().cmp(&b.to_sortable())
            }
            _ => std::cmp::Ordering::Equal,
        }
    }
}

impl std::fmt::Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyType::String(str, _) => {
                write!(f, "{str}")
            }
            KeyType::Integer(nb, _) => {
                write!(f, "{nb}")
            }
            KeyType::UnsignedInteger(nb, _) => {
                write!(f, "{nb}")
            }
            KeyType::Object(obj, _) => {
                write!(f, "{}", obj.to_string())
            }
            _ => write!(f, ""),
        }
    }
}

#[derive(Clone, PartialEq, Props, UiComp)]
pub struct SortTableProps<T: 'static + std::clone::Clone + std::cmp::PartialEq + ToTableData> {
    #[props(extends = caption, extends = GlobalAttributes)]
    attributes: Vec<Attribute>,

    #[props(optional, into)]
    header_class: Option<String>,

    #[props(optional, into)]
    row_class: Option<String>,

    #[props(optional, into)]
    cell_class: Option<String>,

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

pub fn SortTable<T: std::clone::Clone + std::cmp::PartialEq + ToTableData>(
    props: SortTableProps<T>,
) -> Element {
    let mut state = use_signal(|| SortTableState::<T>::new(props.data));

    let header_class = match props.header_class {
        Some(header_class) => tw_merge!(
            "select-none cursor-pointer space-x-2 relative",
            header_class
        ),
        None => "select-none cursor-pointer space-x-2 relative".to_string(),
    };

    let row_class = match props.row_class {
        Some(row_class) => row_class.to_string(),
        None => String::new(),
    };

    let cell_class = match props.cell_class {
        Some(cell_class) => cell_class.to_string(),
        None => String::new(),
    };

    let non_sortable_columns = use_memo(move || {
        let binding = state.read();
        let Some(first_row) = binding.data.first() else {
            return Vec::new();
        };
        first_row
            .to_keytype()
            .iter()
            .enumerate()
            .filter_map(|(index, keytype)| match keytype {
                KeyType::String(_, false) => Some(index),
                KeyType::Element(_, _) => Some(index),
                KeyType::Integer(_, false) => Some(index),
                KeyType::UnsignedInteger(_, false) => Some(index),
                KeyType::Object(_, false) => Some(index),
                _ => None,
            })
            .collect()
    });

    rsx!(
        Table {
            TableHeader {
                TableRow {
                    for (index , head) in T::headers_to_strings().iter().enumerate() {
                        TableHead {
                            class: "{header_class}",
                            onclick: move |_| {
                                if non_sortable_columns.read().contains(&index) {
                                    return;
                                }
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
                            {head.to_string()}
                            if !non_sortable_columns.read().contains(&index)
                                && state.read().get_sorted_col_index() == index
                            {
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
                    TableRow { class: "{row_class}",
                        for field in data.to_keytype().into_iter() {
                            if let KeyType::Element(element, _) = field {
                                TableCell { class: "{cell_class}", {element} }
                            } else {
                                TableCell { class: "{cell_class}", {field.to_string()} }
                            }
                        }
                    }
                }
            }
        }
    )
}
