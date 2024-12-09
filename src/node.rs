use std::{ error::Error, io::{Cursor, Read},};

use crate::utils::crc16_ccitt_false;

const HAS_CHILDREN: [u8;3] = [26, 51, 62];


#[derive(Debug)]
pub enum Value{
    Value(String),
    Nodes(Vec<Node>)
}
#[derive(Debug)]
pub struct Node{
    pub code: u8,
    pub value: Value
}

#[derive(Debug)]
pub struct Nodes{
    pub nodes: Vec<Node>
}

impl Nodes {
    pub fn read_io(cursor: &mut Cursor<&str>, size: u8) -> Result<String, Box<dyn Error>> {
        let mut data = Vec::new();
        data.resize(size as usize, 0);
        match cursor.read_exact(&mut data) {
            Ok(_) => {
                Ok(data.into_iter().map(|x|{
                    char::from(x)
                }).collect::<String>())
            },
            Err(e) => {
                Err(Box::new(e))
            }
        }  
    }
    pub fn verify(&self) -> bool {
        let content = self.dumps();
        let len_content = content.len();
        let computed = crc16_ccitt_false(&content[..len_content-4]);
        let qris_crc = content[len_content-4..].to_string();
        computed == qris_crc
    }
    pub fn dumps(&self) -> String {
        self.nodes.iter().map(|i|{
            i.dumps()
        }).collect::<String>()
    }
    pub fn add_or_update(&mut self, node: Node){
        if let Some(cnode) = self.nodes.iter_mut().find(|cnode|{
            cnode.code == node.code
        }){
            cnode.value = node.value;
        }else{
            self.nodes.push(node);
        }
    }
    pub fn rewrite_crc16(&mut self){
        let dumps = self.dumps().clone();
        let content = crc16_ccitt_false(&dumps[..&dumps.len()-4]);
        self.nodes.iter_mut().for_each(|f|{
            if f.code == 63 {
                f.value = Value::Value(content.clone());
            };
        });
    }
    pub fn set_amount(&mut self, amount: usize){
        self.nodes.iter_mut().for_each(| f| {
            if f.code == 1 || f.code == 58 {
                if f.code == 1 {
                    f.value = Value::Value("12".to_string());

                }else{
                    f.code = 54;
                    f.value = Value::Value(amount.to_string());
                };
            }
        });
    }
    pub fn from_code(code: &str) -> Result<Self, Box<dyn Error>> {
        let node_vec = Nodes::from_code_to_node_vec(code)?;
        match node_vec {
            Value::Nodes(nodes) => {
                Ok(Nodes{nodes})
            },
            Value::Value(val) => {
                Err(val.into())
            }
        }
    }
    pub fn from_code_to_node_vec(code: &str) -> Result<Value, Box<dyn Error>>{
        let mut cursor = Cursor::new(code);
        let mut result: Vec<Node> = Vec::new();
        loop {             
            match Nodes::read_io(&mut cursor, 2) {
                Ok(data) => {
                    let code_in: u8 = data.parse().unwrap();
                    let size = Nodes::read_io(&mut cursor, 2).unwrap();
                    let size_in: u8 = size.parse().unwrap();
                    if HAS_CHILDREN.contains(&code_in) {
                        let children = Nodes::from_code_to_node_vec(&Nodes::read_io(&mut cursor, size_in).unwrap())?;
                        match children {
                            Value::Nodes(nodes) => {
                                result.push(Node { code: code_in, value: Value::Nodes(nodes)});
                            },
                            Value::Value(_)=>{}
                        }
                    }else{
                        result.push(Node{code: code_in, value: Value::Value(Nodes::read_io(&mut cursor, size_in).unwrap())});
                    }
                },
                Err(_) => {
                    break;
                }
            }
        }
        Ok(Value::Nodes(result))
    }
}
impl Node {
    pub fn dumps(&self) -> String{
        let mut dumped = String::new();
        dumped.push_str(&format!("{:0>2}", self.code));
        let result: String=match &self.value {
            Value::Nodes(nodes) => {
                nodes.iter().map(|x|{
                    x.dumps()
                }).collect::<String>()
            },
            Value::Value(val) => {
                format!("{:0>2}{}", val.len(), val)
            }
        };
        if HAS_CHILDREN.contains(&self.code){
            dumped.push_str(&format!("{:0>2}", result.len()));
        }
        dumped.push_str(&result);
        dumped
        
    }
    pub fn add_or_update(&mut self, node: Node) -> Result<(), &str> {
        match &mut self.value {
            Value::Nodes(nodes) => {
                if let Some(cnode) = nodes.iter_mut().find(|cnode|{
                    cnode.code == node.code
                }){
                    cnode.value = node.value;
                }
                Ok(())
            },
            Value::Value(_v) => {
                Err("value is not Nodes type")
            }
        }
    }
}