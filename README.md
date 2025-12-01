# Discord Nekos Bot

23時になると猫耳美少女の画像と共に、「早く寝なさい」と促してくれる Discord Bot です。
画像に合わせて Gemini AI がセリフを生成するため、毎回異なるメッセージを楽しむことができます。

## 機能

- **定時投稿**: 毎日 23:00 (JST) に指定されたチャンネルにメッセージを投稿します。
- **画像取得**: [Nekosia API](https://nekosia.cat/) からランダムな猫耳キャラクターの画像を取得します。
- **AI メッセージ生成**: 取得した画像を Google Gemini API (gemini-2.5-pro) に送信し、キャラクターの雰囲気に合わせた「おやすみメッセージ」や「寝かしつけメッセージ」を生成します。
- **起動時投稿**: Bot 起動時にも動作確認として一度メッセージを送信します。

## 必要要件

- Rust (ローカルでビルドする場合)
- Docker & Docker Compose (コンテナで動かす場合)
- Discord Bot Token
- Google Gemini API Key

## セットアップ

### 1. リポジトリのクローン

```bash
git clone https://github.com/mutyuki/discord-nekos-bot.git
cd discord-nekos-bot
```

### 2. 環境変数の設定

プロジェクトルートに `.env` ファイルを作成し、以下の変数を設定してください。

```env
DISCORD_TOKEN=your_discord_bot_token
DISCORD_CHANNEL_ID=your_channel_id_to_post
GEMINI_API_KEY=your_gemini_api_key
```

| 変数名 | 説明 |
| --- | --- |
| `DISCORD_TOKEN` | Discord Developer Portal から取得した Bot トークン |
| `DISCORD_CHANNEL_ID` | メッセージを送信したい Discord チャンネルの ID (数字) |
| `GEMINI_API_KEY` | Google AI Studio から取得した Gemini API キー |

## 実行方法

### Docker Compose を使用する場合 (推奨)

```bash
docker compose up -d --build
```

### ローカルで実行する場合

```bash
cargo run --release
```

## 開発

### ディレクトリ構成

- `src/main.rs`: Bot のメインロジック、スケジューリング処理
- `src/api.rs`: 外部 API (Nekosia, Gemini) との通信処理

## ライセンス

[MIT License](LICENSE)
