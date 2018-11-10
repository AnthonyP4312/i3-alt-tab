#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate structopt;
extern crate strum;
#[macro_use]
extern crate strum_macros;

use structopt::StructOpt;

pub mod i3 {

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[derive(Deserialize, Debug, Clone)]
    pub struct Node {
        pub name: Option<String>,
        pub id: u64,
        #[serde(rename = "type")]
        pub type_con: Type,
        // pub current_border_width: i32,
        // pub layout: Layout,
        pub rect: Rect,
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

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Rect {
        pub x: i32,
        pub y: i32,
        pub width: i32,
        pub height: i32,
    }

    #[derive(Deserialize, Debug)]
    pub struct Workspace {
        pub num: i8,
        pub visible: bool,
        pub focused: bool,
        pub rect: Rect,
        pub urgent: bool,
    }


    #[derive(EnumString, Debug)]
    pub enum Direction {
        #[strum(serialize="right")]
        Right,
        #[strum(serialize="left")]
        Left,
    }

    #[derive(EnumString, Debug)]
    pub enum WSOptions {
        #[strum(serialize="all")]
        All,
        #[strum(serialize="focused")]
        Focused,
        #[strum(serialize="visible")]
        Visible,
    }

    /// A less pedantic window focusing strategy. Command will focus
    /// windows in a right->down or left->up fashion.
    #[derive(StructOpt, Debug)]
    #[structopt(name = "i3-alt-tab")]
    pub struct Args {
        /// The workspaces to include for tabbing (visible, focused, or all)
        #[structopt(short = "w", long = "workspaces")]
        pub workspaces: WSOptions,

        /// The direction to look for the next window
        #[structopt(short="d", long="direction")]
        pub direction: Direction,

        /// Whether to include floating windows
        #[structopt(short="f", long="floating")]
        pub floating: bool,
    }

}
