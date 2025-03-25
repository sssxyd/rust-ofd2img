use serde::Deserialize;


#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Res{
    color_spaces: ColorSpaces,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
struct ColorSpaces {
    color_space: Vec<ColorSpace>,
}

#[derive(Debug, Deserialize, Default)]
pub(crate) struct ColorSpace {
    #[serde(rename = "ID")]
    id : String,
    #[serde(rename = "Type")]
    color_space_type : String,
}