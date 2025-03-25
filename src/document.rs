use serde::Deserialize;


#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Document{
    common_data: CommonData,
    custom_tags: String,
    annotations: String,
    pages: PageRefs,
}

impl Document {
    pub(crate) fn from_xml(xml: &str) -> Result<Document, serde_xml_rs::Error> {
        serde_xml_rs::from_str(xml)
    }
}

pub(crate) struct PageArea{
    
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
struct CommonData {
    #[serde(rename = "MaxUnitID")]
    max_unit_id: i32,
    public_res: String,
    template_page: PageRef,
    document_res: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
struct PageRefs{
    page : Vec<PageRef>,
}

#[derive(Debug, Deserialize, Default)]
struct PageRef{
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "BaseLoc")]
    base_loc: String,
}


