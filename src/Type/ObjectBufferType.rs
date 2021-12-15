use serde_json::{Deserializer, Serializer};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ObjectBufferJSON {
    pub name : String,
    pub position_x : i8,
    pub position_y : i8,
    pub color : String,
    pub size : i8,
    pub vertex : Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ObjectDataDefineJSON {
    pub objects : Vec<ObjectBufferJSON>,
}
