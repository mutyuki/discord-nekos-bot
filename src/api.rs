use base64::{Engine as _, engine::general_purpose};
use serde::Deserialize;
use serde_json::json;
use std::env;

#[derive(Deserialize, Debug)]
struct NekosiaResponse {
    image: NekosiaImage,
}

#[derive(Deserialize, Debug)]
struct NekosiaImage {
    compressed: NekosiaCompressed,
}

#[derive(Deserialize, Debug)]
struct NekosiaCompressed {
    url: String,
}

#[derive(Deserialize, Debug)]
struct GeminiResponse {
    candidates: Option<Vec<GeminiCandidate>>,
}

#[derive(Deserialize, Debug)]
struct GeminiCandidate {
    content: GeminiContent,
}

#[derive(Deserialize, Debug)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Deserialize, Debug)]
struct GeminiPart {
    text: String,
}

pub async fn fetch_neko_image_url() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.nekosia.cat/api/v1/images/random")
        .await?
        .json::<NekosiaResponse>()
        .await?;
    Ok(response.image.compressed.url)
}

pub async fn fetch_image_bytes(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let image_bytes = reqwest::get(url).await?.bytes().await?;
    Ok(image_bytes.to_vec())
}

pub async fn generate_sleep_message(
    image_bytes: &[u8],
) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY が設定されていません");

    let image_base64 = general_purpose::STANDARD.encode(image_bytes);

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.5-flash:generateContent?key={}",
        api_key
    );

    let client = reqwest::Client::new();

    let prompt_text = "
        この画像のキャラクターになりきって、夜更かししている人に向けて
        「早く寝なさい」「寝ないとだめだよ」「まだ寝ないの？」といった内容のセリフを日本語で一つ生成してください。
        あくまでそういう意図のあるセリフを出力してほしいというだけで、そのままこれらの単語を使う必要はありません。必要に応じて変更してください。

        条件:
        1. 画像のキャラクターの雰囲気（服装、表情、猫耳など）に合わせた口調にすること。
        2. 語尾にはキャラクターに合わせて「にゃ」「だよ」「です」などを自然につけること(これはあくまで一例でキャラに合わせて適宜変えてほしい)。
        3. 50文字以内の短い文で出力すること。
        4. セリフのみを出力し、「」などの記号は不要。
        5. URLなどは絶対に出力しないこと。
        6.オリジナリティが出るようにキャラの特徴をしっかり掴むこと
    ";

    let payload = json!({
        "contents": [{
            "parts": [
                { "text": prompt_text },
                {
                    "inline_data": {
                        "mime_type": "image/jpeg",
                        "data": image_base64
                    }
                }
            ]
        }]
    });

    let res = client.post(&url).json(&payload).send().await?;

    if !res.status().is_success() {
        let err_text = res.text().await?;
        return Err(format!("Gemini API Error: {}", err_text).into());
    }

    let response_body: GeminiResponse = res.json().await?;
    if let Some(text) = response_body
        .candidates
        .as_ref()
        .and_then(|v| v.first())
        .and_then(|c| c.content.parts.first())
        .map(|p| p.text.clone())
    {
        Ok(text)
    } else {
        Err("Geminiからの応答にテキストが含まれていませんでした（セーフティ等）".into())
    }
}
