use std::fs::File;
use std::io::BufReader;

use zip::ZipArchive;
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct OfdNode {
    #[serde(rename = "DocBody")]
    pub doc_body: DocBody,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DocBody {
    pub doc_info: DocInfo,
    pub doc_root: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DocInfo {
    pub doc_id: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    #[serde(rename="Abstract")]
    pub abstract_text: Option<String>,
    pub creation_date: Option<String>,
    pub mod_date: Option<String>,
    pub doc_usage: Option<String>,
    pub cover: Option<String>,
    pub keywords: Option<KeywordList>,
    pub creator: Option<String>,
    pub creator_version: Option<String>,
    pub custom_datas: Option<CustomDataList>,
}

impl DocInfo {
    pub fn to_map(&self) -> HashMap<String, Any> {
        let mut map = HashMap::new();

        // 处理字符串类型字段
        self.add_field(&mut map, "DocId", &self.doc_id);
        self.add_field(&mut map, "Title", &self.title);
        self.add_field(&mut map, "Author", &self.author);
        self.add_field(&mut map, "Subject", &self.subject);
        self.add_field(&mut map, "Abstract", &self.abstract_text);
        self.add_field(&mut map, "CreationDate", &self.creation_date);
        self.add_field(&mut map, "ModDate", &self.mod_date);
        self.add_field(&mut map, "DocUsage", &self.doc_usage);
        self.add_field(&mut map, "Cover", &self.cover);
        self.add_field(&mut map, "Creator", &self.creator);
        self.add_field(&mut map, "CreatorVersion", &self.creator_version);

        // 处理需要序列化的复杂类型字段
        self.add_serialized_field(&mut map, "Keywords", &self.keywords.to_list());
        self.add_serialized_field(&mut map, "CustomDatas", &self.custom_datas.to_map());

        map
    }

    fn add_field(&self, map: &mut HashMap<String, Any>, key: &str, value: &Option<String>) {
        if let Some(val) = value {
            map.insert(key.to_string(), val.clone());
        }
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct KeywordList {
    pub keyword: Vec<Keyword>
}

impl KeywordList {
    pub to_list(&self) -> Vec<String> {
        self.keywords.iter().map(|k| k.value.clone()).collect()
    }
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct Keyword {
    #[serde(rename = "$value")]
    pub value: String
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct CustomDataList {
    #[serde(rename = "CustomData")]
    pub custom_data: Vec<CustomData>,
}

impl CustomDataList {
    pub fn to_map(&self) -> HashMap<String, String> {
        self.custom_data
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
            .collect()
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub struct CustomData {
    pub name: Option<String>, // XML attribute
    #[serde(rename = "$value")]
    pub value: String, // XML node's text content
}

impl OfdNode {
    pub fn from_xml(xml: &str) -> Result<OfdNode, serde_xml_rs::Error> {
        serde_xml_rs::from_str(xml)
    }
}

#[derive(Debug)]
pub struct OfdDoc {
    pub node: OfdNode,
    pub zip_archive: ZipArchive<BufReader<File>>,
}

impl OfdDoc {
    pub fn open(file_path: &str) -> Result<OfdDoc, Error> {
        let file = File::open(file_path)?;
        let reader = io::BufReader::new(file);
        let mut zip = ZipArchive::new(reader)?;
    
        let mut content = String::new();
    
        // Find the OFD.xml file and parse the content to ofd object.
        {
            let mut ofd_file = zip.by_name("OFD.xml")?;
            ofd_file.read_to_string(&mut content)?;
        }
    
        // Parse the XML content into an OfdNode.
        let ofd_node = OfdNode::from_xml(&content)?;
    
        let ofd_result = OfdDoc {
            node: ofd_node,
            zip_archive: zip,
        };
    
        Ok(ofd_result)
    }

    pub fn get_attributes(&self) -> HashMap<String, Any> {
        return self.node.doc_body.doc_info.to_map()
    } 
}
