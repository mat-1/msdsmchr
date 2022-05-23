use image::DynamicImage;
use serde::Deserialize;

#[derive(Deserialize)]
struct MojangSkinResponse {
    pub properties: Vec<MojangSkinProperty>,
}
#[derive(Deserialize)]
struct MojangSkinProperty {
    pub name: String,
    pub value: String,
}

#[derive(Deserialize)]
struct TexturesData {
    pub textures: TexturesDataTextures,
}
#[derive(Deserialize)]
struct TexturesDataTextures {
    #[serde(rename = "SKIN")]
    pub skin: TexturesDataTexturesSkin,
}
#[derive(Deserialize)]
struct TexturesDataTexturesSkin {
    pub url: String,
}

pub async fn download_from_uuid(uuid: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://sessionserver.mojang.com/session/minecraft/profile/{}",
        uuid
    );
    let resp = reqwest::get(&url).await?;
    let body = resp.text().await?;
    let json: MojangSkinResponse = serde_json::from_str(&body)?;
    let skin_base64 = &json
        .properties
        .iter()
        .find(|p| p.name == "textures")
        .unwrap()
        .value;

    let skin_data_bytes = base64::decode(skin_base64)?;
    let skin_data = std::str::from_utf8(&skin_data_bytes)?;

    let skin_textures_data: TexturesData = serde_json::from_str(skin_data)?;
    let skin_url = &skin_textures_data.textures.skin.url;

    // get the last part of the url
    let skin_url_parts: Vec<&str> = skin_url.split("/").collect();
    let texture_id = skin_url_parts.last().unwrap();

    download_from_texture_id(texture_id).await
}

pub async fn download_from_texture_id(
    texture_id: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let url = format!("https://textures.minecraft.net/texture/{}", texture_id);
    let resp = reqwest::get(&url).await?;
    let body = resp.bytes().await?;
    Ok(body.to_vec())
}

pub async fn download_from_id(id: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // figure out which id type it is based on the length
    // 32 is a uuid
    // 64 is a texture id
    match id.len() {
        32 => download_from_uuid(id).await,
        _ => download_from_texture_id(id).await,
    }
}
