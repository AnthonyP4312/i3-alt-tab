#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

pub mod i3 {

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Node {
        id: u8,
        name: String,
        #[serde(rename = "type")]
        type_con: Type,
        border: String,

    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Type {
        #[serde(rename_all = "snake_case")]
        Root,
        Output,
        Con,
        FloatingCon,
        Workspace,
        Dockarea
    }

}
