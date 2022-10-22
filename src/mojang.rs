use lazy_static::lazy_static;
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

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

#[derive(Debug)]
pub enum DownloadError {
    InvalidTexture,
    FetchError,
    InvalidUuid,
}

impl Display for DownloadError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            DownloadError::InvalidTexture => write!(f, "Invalid texture"),
            DownloadError::FetchError => write!(f, "Error fetching"),
            DownloadError::InvalidUuid => write!(f, "Invalid UUID"),
        }
    }
}

impl From<reqwest::Error> for DownloadError {
    fn from(_: reqwest::Error) -> Self {
        DownloadError::FetchError
    }
}

lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

pub async fn download_from_uuid(uuid: &str) -> Result<Vec<u8>, DownloadError> {
    let url = format!(
        "https://sessionserver.mojang.com/session/minecraft/profile/{}",
        uuid
    );
    let resp = CLIENT.get(&url).send().await?;
    let json: MojangSkinResponse = resp.json().await?;
    let skin_base64 = &json
        .properties
        .iter()
        .find(|p| p.name == "textures")
        .unwrap()
        .value;

    let skin_data_bytes = base64::decode(skin_base64).expect("Invalid base64");
    let skin_data = std::str::from_utf8(&skin_data_bytes).expect("Invalid UTF-8");

    let skin_textures_data: TexturesData = serde_json::from_str(skin_data).expect("Invalid JSON");
    let skin_url = &skin_textures_data.textures.skin.url;

    // get the last part of the url
    let skin_url_parts: Vec<&str> = skin_url.split('/').collect();
    let texture_id = skin_url_parts.last().unwrap();

    download_from_texture_id(texture_id).await
}

pub async fn download_from_texture_id(texture_id: &str) -> Result<Vec<u8>, DownloadError> {
    let url = format!("https://textures.minecraft.net/texture/{}", texture_id);
    let resp = CLIENT.get(&url).send().await?;
    if resp.status() != 200 {
        return Err(DownloadError::InvalidTexture);
    }
    let body = resp.bytes().await?;
    Ok(body.to_vec())
}

pub async fn download_from_id(id: &str) -> Result<Vec<u8>, DownloadError> {
    // figure out which id type it is based on the length
    // 32 is a uuid
    // 64 is a texture id
    match id.len() {
        32 => {
            let uuid = Uuid::parse_str(id).map_err(|_| DownloadError::InvalidUuid)?;
            Ok(match download_from_uuid(id).await {
                Ok(data) => data,
                Err(_) => {
                    // random skin depending on the least significant bit of the uuid
                    match java_hash_code(&uuid) & 1 {
                        0 => include_bytes!("assets/steve.png").to_vec(),
                        _ => include_bytes!("assets/alex.png").to_vec(),
                    }
                }
            })
        }
        _ => download_from_texture_id(id).await,
    }
}

fn java_hash_code(uuid: &Uuid) -> u32 {
    let most_sig_bits = uuid.as_u128() >> 64;
    let least_sig_bits = uuid.as_u128() & 0xFFFF_FFFF_FFFF_FFFF;
    let hash = most_sig_bits ^ least_sig_bits;
    (hash ^ (hash >> 32)) as u32
}
