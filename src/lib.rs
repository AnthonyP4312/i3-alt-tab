#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

pub mod i3 {

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Node {
        pub name: Option<String>,
        // pub id: u64,
        #[serde(rename = "type")]
        pub type_con: Type,
        // pub current_border_width: i32,
        // pub layout: Layout,
        // pub rect: Rect,
        pub focused: bool,
        // pub floating_nodes: Vec<Node>,
        pub window: Option<u64>,
        pub nodes: Vec<Node>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "snake_case")]
    pub enum Type {
        Root,
        Output,
        Con,
        FloatingCon,
        Workspace,
        Dockarea,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "snake_case")]
    pub enum Layout {
        Splith,
        Splitv,
        Stacked,
        Tabbed,
        Dockarea,
        Output
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Rect {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    }

}
