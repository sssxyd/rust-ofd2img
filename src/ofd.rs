use std::{collections::HashMap, fs::File};
use std::io::{self, BufReader, Read};

use serde::ser::{SerializeMap, SerializeSeq};
use thiserror::Error;
use zip::ZipArchive;
use serde::{Deserialize, Serialize, Serializer};

use crate::document::Document;

#[derive(Debug)]
pub enum Value {
    String(String),
    ListString(Vec<String>),
    MapString(HashMap<String, String>),
    LisMapString(Vec<HashMap<String, String>>),
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Value::String(s) => serializer.serialize_str(s),
            Value::ListString(v) => {
                // 创建序列化器并逐个添加元素
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for item in v {
                    let ret = seq.serialize_element(item);
                    if let Err(e) = ret {
                        return Err(e);
                    }
                }
                seq.end() // 返回最终结果
            }
            Value::MapString(m) => {
                // 创建Map序列化器并逐个添加键值对
                let mut map = serializer.serialize_map(Some(m.len()))?;
                for (k, v) in m {
                    let ret = map.serialize_entry(k, v);
                    if let Err(e) = ret {
                        return Err(e);
                    }
                }
                map.end() // 返回最终结果
            }
            Value::LisMapString(v) => {
                let mut seq = serializer.serialize_seq(Some(v.len()))?;
                for item in v {
                    let ret = seq.serialize_element(item);
                    if let Err(e) = ret {
                        return Err(e);
                    }
                }
                seq.end()
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum OfdError {
    #[error("Zip error: {0}")]
    ZipError(zip::result::ZipError),
    #[error("IO error: {0}")]
    IoError(io::Error),
    #[error("Serde XML error: {0}")]
    SerdeXmlError(serde_xml_rs::Error),
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
struct OfdNode {
    #[serde(rename = "DocBody")]
    doc_body: DocBody,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
struct DocBody {
    doc_info: DocInfo,
    doc_root: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default, rename_all = "PascalCase")]
struct DocInfo {
    doc_id: String,
    title: String,
    author: String,
    subject: String,
    #[serde(rename="Abstract")]
    abstract_text: String,
    creation_date: String,
    mod_date: String,
    doc_usage: String,
    cover: String,
    keywords: Option<KeywordList>,
    creator: String,
    creator_version: String,
    custom_datas: Option<CustomDataList>,
}

impl DocInfo {
    fn attributes(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("DocId".to_string(), self.doc_id.clone());
        map.insert("Title".to_string(), self.title.clone());
        map.insert("Author".to_string(), self.author.clone());
        map.insert("Subject".to_string(), self.subject.clone());
        map.insert("Abstract".to_string(), self.abstract_text.clone());
        map.insert("CreationDate".to_string(), self.creation_date.clone());
        map.insert("ModDate".to_string(), self.mod_date.clone());
        map.insert("DocUsage".to_string(), self.doc_usage.clone());
        map.insert("Cover".to_string(), self.cover.clone());
        map.insert("Creator".to_string(), self.creator.clone());
        map.insert("CreatorVersion".to_string(), self.creator_version.clone());
        map.insert("Keywords".to_string(), self.keywords.as_ref().map_or("".to_string(), |k| k.to_list().join(",")));
        map
    }

    fn custom_datas(&self) -> HashMap<String, String> {
        return self.custom_datas.as_ref().map_or(HashMap::new(), |c| c.to_map());
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
struct KeywordList {
    keyword: Vec<Keyword>
}

impl KeywordList {
    fn to_list(&self) -> Vec<String> {
        return self.keyword.iter().map(|k| k.value.clone()).collect();
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
struct Keyword {
    #[serde(rename = "$value")]
    value: String
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
struct CustomDataList {
    #[serde(rename = "CustomData")]
    custom_data: Vec<CustomData>,
}

impl CustomDataList {
    fn to_map(&self) -> HashMap<String, String> {
        return self.custom_data
            .iter()
            .filter_map(|data| {
                // 处理空name的情况（XML attribute可能不存在）
                data.name.as_ref().map(|name| {
                    // 处理可能的空字符串name
                    (name.clone(), data.value.clone())
                })
            })
            // 处理重复key的情况（保留最后一个出现的值）
            .rev()
            .collect();
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
struct CustomData {
    name: Option<String>, // XML attribute
    #[serde(rename = "$value")]
    value: String, // XML node's text content
}

impl OfdNode {
    fn from_xml(xml: &str) -> Result<OfdNode, serde_xml_rs::Error> {
        serde_xml_rs::from_str(xml)
    }
}

#[derive(Debug)]
pub struct OfdDoc {
    node: OfdNode,
    zip_archive: ZipArchive<BufReader<File>>,
    document: Document,
    pub attributes: HashMap<String, String>,
    pub custom_datas: HashMap<String, String>,
}

impl OfdDoc {
    pub fn open(file_path: &str) -> Result<OfdDoc, OfdError> {
        let file = File::open(file_path).map_err(OfdError::IoError)?;
        let reader = io::BufReader::new(file);
        let mut zip = ZipArchive::new(reader).map_err(OfdError::ZipError)?;
    
        let mut content = String::new();
    
        // Find the OFD.xml file and parse the content to ofd object.
        {
            let mut ofd_file = zip.by_name("OFD.xml").map_err(OfdError::ZipError)?;
            ofd_file.read_to_string(&mut content).map_err(OfdError::IoError)?;
        }
    
        // Parse the XML content into an OfdNode.
        let ofd_node: OfdNode = OfdNode::from_xml(&content).map_err(OfdError::SerdeXmlError)?;

        content.clear();
        {
            let mut doc_file = zip.by_name(&ofd_node.doc_body.doc_root).map_err(OfdError::ZipError)?;
            doc_file.read_to_string(&mut content).map_err(OfdError::IoError)?;
        }

        let document: Document = Document::from_xml(&content).map_err(OfdError::SerdeXmlError)?;

        let attributes = ofd_node.doc_body.doc_info.attributes();
        let custom_datas = ofd_node.doc_body.doc_info.custom_datas();
    
        let ofd_result = OfdDoc {
            node: ofd_node,
            zip_archive: zip,
            document,
            attributes,
            custom_datas,
        };
    
        Ok(ofd_result)
    }

    pub fn info(&self) -> String {
        let mut map: HashMap<String, HashMap<String,String>> = HashMap::new();
        map.insert("attributes".to_string(), self.attributes.clone());
        map.insert("custom_datas".to_string(), self.custom_datas.clone());
        serde_json::to_string(&map).unwrap()
    }
}
