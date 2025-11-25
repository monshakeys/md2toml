# md2toml - 一鍵批量轉換 Claude Slash Commands

**終結手動轉檔的煩惱。** 專為 Claude Code 使用者打造的批量轉換工具，將整個資料夾的 Markdown slash commands 瞬間轉成 TOML 格式。

## Features

- 🚀 **批量處理整個資料夾** - 不用再一個一個檔案慢慢轉，拖進去就完成。遞迴處理所有子資料夾，保持原本的目錄結構。

- 🎯 **聰明過濾垃圾檔案** - 自動跳過草稿檔（`~draft.md`）、備份資料夾（`~old/`）、隱藏系統檔（`.DS_Store`、`.git/`）。不用手動清理，不會轉出一堆你不想要的檔案。

- 🛡️ **單檔出錯不炸全部** - 34 個檔案轉換，就算 8 個有問題，其他 26 個照樣成功。給你完整的錯誤報告，不會轉到一半就放棄。

- ⚡ **自動語法轉換** - `$ARGUMENTS` 自動變 `{{ args }}`，`` !`command` `` 自動變 `!{command}`。不用記語法差異，不會手殘打錯。

- 🔒 **安全確認機制** - 要覆蓋現有資料夾時會先問你，不會靜悄悄把你的檔案蓋掉。

## Installation

### Homebrew (Recommended)

```bash
brew install monshakeys/tap/md2toml
```

### Alternative Methods

- Download the latest binary from the [releases page](https://github.com/monshakeys/md2toml/releases) and place it in your `$PATH`.
- Or these methods:

```bash
# Clone the repository
git clone git@github.com:monshakeys/md2toml.git
cd md2toml

# Build for development
cargo build
./target/debug/md2toml

# Build for production
cargo build --locked --release
./target/release/md2toml

# Run without building an executable
cargo run              # Debug mode
cargo run --release    # Release mode

# Install to local cargo path
cargo install --path .
```

## Usage

### 批量轉換整個資料夾

```bash
# 最簡單的用法 - 轉換整個 commands 資料夾
md2toml commands/

# 輸出到指定位置
md2toml commands/ -o output/

# 顯示詳細過程（看到每個檔案的轉換狀態）
md2toml commands/ --verbose
```

### 單檔轉換

```bash
# 轉換單一檔案
md2toml input.md              # 產生 input.toml

# 指定輸出檔名
md2toml input.md -o custom.toml
```

### 聰明過濾範例

```bash
# 你的資料夾結構：
commands/
├── review.md           # ✅ 會轉換
├── ~draft.md           # ❌ 自動跳過（草稿）
├── .DS_Store           # ❌ 自動跳過（系統檔）
├── dev/
│   └── debug.md        # ✅ 會轉換
├── ~backup/
│   └── old.md          # ❌ 自動跳過（備份資料夾）
└── .git/
    └── config.md       # ❌ 自動跳過（隱藏資料夾）

# 一行指令，只轉換你真正需要的檔案
md2toml commands/ -o output/
```

**你不需要：**
- ❌ 手動清理草稿檔
- ❌ 刪除 .DS_Store
- ❌ 擔心轉換到 .git 資料夾
- ❌ 先備份再轉換

**你只要：**
- ✅ 用 `~` 標記草稿檔案
- ✅ 一行指令搞定

## Output Example

```
Converting directory: "commands" -> "output/"

=== Conversion Summary ===
Total files found:     34
Successfully converted: 26
Failed:                8

Failed files:
  ✗ "commands/broken.md"
    Error: Invalid frontmatter format: This is not a markdown file for cc slash command.
```

**你會得到：**
- 清楚知道有幾個檔案成功
- 哪些檔案有問題，什麼原因
- 成功的檔案已經在 output/ 資料夾等你了

## 轉換範例

### 輸入 (review.md)
```markdown
---
title: Code Review
description: Review code with AI
author: DevTeam
---

Please review $ARGUMENTS carefully.

Run !`git diff` to see changes.
```

### 輸出 (review.toml)
```toml
author = "DevTeam"
description = "Review code with AI"
title = "Code Review"

prompt = """
Please review {{ args }} carefully.

Run !{git diff} to see changes.
"""
```

**自動完成：**
- ✅ YAML 轉 TOML 格式
- ✅ `$ARGUMENTS` → `{{ args }}`
- ✅ `` !`git diff` `` → `!{git diff}`
- ✅ Metadata 按字母排序
- ✅ 保留所有換行、縮排、格式

## 必要欄位檢查

```bash
# ❌ 這會失敗（缺少 description）
---
title: Example
---

# ✅ 這會成功
---
title: Example
description: A valid command
---
```

**為什麼要檢查？** 因為 Claude Code 的 slash command 必須有 title 和 description，與其轉換後才發現不能用，不如在轉換時就告訴你哪些檔案有問題。

## 進階用法

### 用 `~` 管理草稿

```bash
# 你的工作流程：
# 1. 寫新 command 時用 ~ 開頭
commands/
├── review.md           # 正式版
└── ~review-v2.md       # 新版本草稿

# 2. 轉換時自動跳過草稿
md2toml commands/ -o output/

# 3. 草稿完成後，移除 ~ 即可
mv commands/~review-v2.md commands/review.md
```

### 批量轉換大型專案

```bash
# 有 100+ 個 slash commands？沒問題
md2toml .claude/commands/ --verbose

# 即使有 20 個檔案格式錯誤，其他 80 個照樣轉換成功
# 你會看到詳細的錯誤報告，知道哪些檔案需要修正
```

## Project Structure

```
md2toml/
├── Cargo.toml
├── src/
│   ├── main.rs         # CLI 介面
│   ├── lib.rs          # 函式庫入口
│   ├── parser.rs       # YAML frontmatter 解析 + 驗證
│   ├── replacer.rs     # 語法轉換（$ARGUMENTS, !`cmd`）
│   ├── generator.rs    # TOML 輸出生成
│   ├── directory.rs    # 批量處理 + 智慧過濾
│   └── error.rs        # 錯誤處理
├── tests/              # 整合測試
└── fixtures/           # 測試資料
```

## Why md2toml?

### 你遇過這些問題嗎？

- ❌ 有 50 個 slash commands 要轉成 TOML，一個一個複製貼上到瘋掉
- ❌ 轉換時把 `.DS_Store` 和 `~draft.md` 也轉進去了
- ❌ 手動改 `$ARGUMENTS` 改到手酸，還容易漏掉
- ❌ 轉到第 30 個檔案時發現一個格式錯誤，前面 29 個白轉了
- ❌ 不小心覆蓋了重要的 output 資料夾

### md2toml 解決方案：

- ✅ **一行指令，全部搞定** - `md2toml commands/ -o output/`
- ✅ **自動過濾垃圾** - 草稿、備份、隱藏檔全部自動跳過
- ✅ **語法自動轉換** - $ARGUMENTS 和 !`command` 自動處理
- ✅ **部分失敗不影響** - 錯誤檔案列出來，成功的照樣完成
- ✅ **覆蓋前先確認** - 不會靜悄悄毀掉你的檔案

## 實際使用場景

### 場景 1: 遷移 Claude Code 專案

```bash
# 你有 100+ 個 markdown slash commands 要轉成 TOML
# 手動轉？開玩笑嗎？

md2toml .claude/commands/ --verbose

# 2 秒後：
# ✅ 95 個成功
# ❌ 5 個格式錯誤（有詳細報告）
# 完工！
```

### 場景 2: 開發新 Commands

```bash
# 你在開發新功能，有很多草稿版本
commands/
├── feature.md          # 正式版
├── ~feature-v2.md      # 實驗版本
├── ~feature-old.md     # 舊版備份
└── ~temp.md            # 臨時筆記

# 轉換時只要正式版，不用手動刪除草稿
md2toml commands/ -o output/
# 只會轉換 feature.md
```

### 場景 3: 多人協作專案

```bash
# Git 會產生 .git/，編輯器會產生 .vscode/
# macOS 會產生 .DS_Store
# 這些都不應該被轉換

md2toml shared-commands/ -o production/
# 自動跳過所有系統檔案和版本控制資料夾
```

## 技術細節

### 語法轉換規則

| Markdown (YAML)  | TOML              | 說明                        |
|------------------|-------------------|-----------------------------|
| `$ARGUMENTS`     | `{{ args }}`      | Claude Code 參數語法        |
| `` !`command` `` | `!{command}`      | Shell 指令執行語法          |

### 必要欄位驗證

為了確保轉出來的 TOML 能在 Claude Code 使用，每個檔案必須有：
- `title`: Command 的名稱
- `description`: Command 的說明

### 智慧過濾規則

**波浪符號 (`~`)**：標記臨時內容
- 用途：草稿、備份、實驗版本
- 範例：`~draft.md`、`~backup/`、`~wip-feature.md`

**句點符號 (`.`)**：跳過系統檔案
- 用途：隱藏檔案、版本控制、編輯器設定
- 範例：`.DS_Store`、`.git/`、`.vscode/`、`.claude/`

**不會被過濾**：
- ✅ `file.with.dots.md` - 中間有句點的檔案
- ✅ `feature~old~backup.md` - 中間有波浪符號的檔案

## 常見問題

**Q: 我有 100 個檔案，會不會很慢？**
A: 不會。工具使用高效能過濾，跳過不需要的資料夾時完全不進去讀取。

**Q: 如果有些檔案格式錯誤怎麼辦？**
A: 錯誤的檔案會被記錄下來，但不會影響其他檔案的轉換。你會得到完整的錯誤報告，知道哪些檔案需要修正。

**Q: 會不會覆蓋我現有的檔案？**
A: 如果輸出資料夾已經存在，會先詢問你是否要覆蓋。不會無聲無息把你的檔案蓋掉。

**Q: 我的檔案有很複雜的格式，會不會跑掉？**
A: 不會。所有的換行、縮排、空白、特殊字元都會完整保留。

**Q: 為什麼要用這個工具？直接手動改不行嗎？**
A:
- 1 個檔案：手動改可以
- 10 個檔案：開始覺得煩
- 50 個檔案：手動改到懷疑人生
- 100+ 個檔案：**你需要 md2toml**

## 快速開始

```bash
# 1. 安裝
cargo build --release

# 2. 使用（就這麼簡單）
./target/release/md2toml commands/ -o output/

# 3. 完工！
# 去 output/ 資料夾看結果
```

## 實際輸出範例

### 成功案例

輸入 (commands/review.md):
```markdown
---
title: Code Review
description: Review code changes
---

Review $ARGUMENTS and provide feedback.
Use !`git diff` to check changes.
```

輸出 (output/review.toml):
```toml
description = "Review code changes"
title = "Code Review"

prompt = """
Review {{ args }} and provide feedback.
Use !{git diff} to check changes.
"""
```

### 失敗案例（有詳細錯誤訊息）

輸入 (commands/broken.md):
```markdown
---
title: Broken Command
---

Missing description field!
```

輸出：
```
✗ "commands/broken.md"
  Error: Invalid frontmatter format: This is not a markdown file for cc slash command.
```

**為什麼失敗？** 缺少必要的 `description` 欄位。修正後重新轉換即可。

## 真實使用情境

### 場景：遷移現有的 Claude Code 專案

你有一個專案，`.claude/commands/` 資料夾裡有 50 個 markdown slash commands。Claude Code 更新後要改用 TOML 格式。

**不用 md2toml：**
1. 開啟第 1 個檔案
2. 複製 frontmatter，轉成 TOML 格式
3. 手動改 `$ARGUMENTS` → `{{ args }}`
4. 找到所有的 `` !`command` ``，改成 `!{command}`
5. 存檔
6. 重複 49 次
7. **估計耗時：2-3 小時**
8. **出錯機率：極高**（手動改 50 個檔案，誰不會打錯？）

**使用 md2toml：**
1. `md2toml .claude/commands/ --verbose`
2. **耗時：2 秒**
3. **出錯機率：0**（程式不會手殘）

### 場景：團隊協作開發 Commands

團隊有 5 個人在開發 slash commands，每個人都有自己的草稿版本。

```bash
shared-commands/
├── feature-a.md        # 正式版
├── ~feature-a-v2.md    # Alice 的實驗版本
├── ~feature-a-test.md  # Bob 的測試版本
├── feature-b.md        # 正式版
└── .git/               # Git 版本控制

# 轉換時只要正式版
md2toml shared-commands/ -o production/
# 只會轉換 feature-a.md 和 feature-b.md
# 草稿和 Git 資料夾自動跳過
```

## 為什麼選擇 md2toml？

| 痛點 | 手動處理 | md2toml |
|------|---------|---------|
| 轉換 50 個檔案 | 2-3 小時 | 2 秒 |
| 過濾草稿檔案 | 手動刪除或小心避開 | 自動跳過 |
| 語法轉換 | 每個檔案手動找+改 | 自動處理 |
| 一個檔案錯誤 | 可能整批重做 | 其他檔案照樣成功 |
| 覆蓋風險 | 記得備份... | 會先詢問確認 |
| 格式跑掉 | 小心翼翼複製貼上 | 完整保留 |

## 誰需要這個工具？

- ✅ **Claude Code 重度使用者** - 有大量 slash commands 要管理
- ✅ **團隊協作** - 多人開發 commands，需要過濾草稿和備份
- ✅ **專案遷移** - 從舊格式轉到新格式
- ✅ **效率控** - 不想把時間浪費在重複性工作

## 官方格式參考

- Gemini CLI: [Gemini CLI Custom Commands](https://github.com/google-gemini/gemini-cli/blob/main/docs/cli/custom-commands.md)
- Claude Code: [Claude Code Custom Slash Commands](https://code.claude.com/docs/en/slash-commands#custom-slash-commands)
