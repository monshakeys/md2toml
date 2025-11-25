# md2toml 轉換規則

## 檔案處理規則（由外而內）

### 規則 1: 檔案和資料夾過濾 🆕
遍歷資料夾時，首先執行過濾規則。

- **忽略規則**：
  - 檔案名稱以 `~` 或 `.` 開頭的 `.md` 檔案會被忽略
  - 資料夾名稱以 `~` 或 `.` 開頭的整個資料夾會被忽略（包含其中的所有檔案）
- **目的**：
  - 波浪符號 (`~`)：方便使用者標記臨時檔案或草稿（例如：`~draft.md`、`~backup.md`）
  - 句點符號 (`.`)：跳過隱藏檔案和系統資料夾（例如：`.DS_Store`、`.git`、`.claude`）
- **性能考量**：
  - 使用 `walkdir` 的 `filter_entry` 在遍歷時就過濾，避免不必要的讀取
  - 同時過濾檔案和資料夾，效率最佳
- **範例**：
  - ✅ 會處理：`normal.md`、`docs/guide.md`、`file~with~tilde.md`、`file.with.dots.md`
  - ❌ 會忽略：`~temp.md`、`.hidden.md`、`~backup/readme.md`、`.git/config.md`、`docs/~old/guide.md`

## 單檔轉換規則（由上而下）

### 規則 2: YAML Frontmatter 解析
解析檔案的 metadata 區塊。

- **輸入**：`---` 之間的 YAML 格式 metadata
- **輸出**：`BTreeMap<String, String>`（自動按字母排序）
- **特殊處理**：
  - 數字自動轉字串（`42` → `"42"`）
  - 布林值自動轉字串（`true` → `"true"`）

### 規則 3: 必要欄位驗證 ⚠️
驗證檔案是否為有效的 cc slash command 檔案。

- **必要欄位**：`title` 和 `description`（區分大小寫）
- **錯誤訊息**：`"This is not a markdown file for cc slash command."`
- **目的**：確保檔案是專門給 cc slash command 使用的

### 規則 4: Content 提取
提取 frontmatter 之後的所有內容。

- **輸入**：第二個 `---` 之後的所有內容
- **保留**：
  - 所有換行（除了最後一個）
  - 所有縮排
  - 所有空白行
  - 所有特殊字元

## 內容替換規則（由特殊到一般）

### 規則 5: $ARGUMENTS 替換
替換特定的參數佔位符。

- **輸入**：`$ARGUMENTS`（區分大小寫）
- **輸出**：`{{ args }}`
- **支援**：多次出現、多行內容

### 規則 6: Backtick Pattern 替換
替換命令模式。

- **輸入**：`` !`command` ``
- **輸出**：`!{command}`
- **正則表達式**：`!`([^`]*)``
- **效能優化**：使用 `once_cell` 靜態編譯，避免每次呼叫都重新編譯

## 輸出生成規則

### 規則 7: TOML 生成
生成最終的 TOML 格式輸出。

- **Metadata**：
  - 格式：`key = "value"`
  - 順序：按字母排序（BTreeMap 自動處理）
  - 特殊字元：雙引號要轉義（`"` → `\"`）

- **Content**：
  - 格式：`prompt = """..."""`（多行字串）
  - 保留所有格式

---

## 執行順序

### 資料夾轉換流程
```
┌────────────────────────────────────────────────┐
│ 1. collect_md_files()                          │
│    └─ 遍歷資料夾（遞迴）                       │
│    └─ filter_entry: 過濾 ~ 或 . 開頭的檔案   │
│    └─ filter_entry: 跳過 ~ 或 . 開頭的資料夾 │
│    └─ 收集所有 .md 檔案                        │
└────────────────────────────────────────────────┘
                    ↓
          ┌────────────────┐
          │ 逐一轉換每個檔案 │
          └────────────────┘
                    ↓
       （進入單檔轉換流程）
```

### 單檔轉換流程
```
┌─────────────────────────────────────────┐
│ 1. parse_markdown()                     │
│    └─ 一次掃描完成兩項工作：             │
│       • 解析 YAML frontmatter           │
│       • 提取 content                    │
│    └─ 返回 ParsedMarkdown 結構體       │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│ 2. validate_required_fields()           │
│    └─ 檢查 "title" 存在                │
│    └─ 檢查 "description" 存在          │
│    └─ 失敗則拋出錯誤                   │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│ 3. apply_all_replacements()             │
│    └─ replace_arguments()               │
│       ($ARGUMENTS → {{ args }})        │
│    └─ replace_backtick_patterns()       │
│       (!`cmd` → !{cmd})                │
└─────────────────────────────────────────┘
                  ↓
┌─────────────────────────────────────────┐
│ 4. generate_toml()                      │
│    └─ 寫入 metadata (按字母排序)       │
│    └─ 寫入 prompt = """..."""          │
└─────────────────────────────────────────┘
```

### 詳細步驟說明

#### Directory: Collect Files（資料夾遍歷）
```rust
let md_files = collect_md_files(src_dir)?;
```

**關鍵特性**：
- 使用 `filter_entry` 在遍歷階段就過濾，提升效能
- 跳過整個 `~` 或 `.` 開頭的資料夾，避免不必要的遞迴

#### Step 1: Parse（解析）
```rust
// 優化版：一次掃描完成兩件事
let ParsedMarkdown { metadata, content } = parse_markdown(input)?;

// 向後相容：分開呼叫
let metadata = parse_frontmatter(input)?;
let content = extract_content(input)?;
```

**關鍵優化**：只掃描一次 `---` 分隔符，避免重複遍歷。

#### Step 2: Validate（驗證）
```rust
validate_required_fields(&metadata)?;
```

**目的**：確保檔案包含 `title` 和 `description` 欄位。

#### Step 3: Replace（替換）
```rust
// 鏈式調用（內部順序固定）
let processed = apply_all_replacements(content);

// 等同於
let step1 = replace_arguments(content);         // $ARGUMENTS → {{ args }}
let step2 = replace_backtick_patterns(&step1);  // !`cmd` → !{cmd}
```

**執行順序**：先處理 `$ARGUMENTS`，再處理 backtick patterns。

#### Step 4: Generate（生成）
```rust
let toml_output = generate_toml(metadata, processed_content)?;
```

**輸出格式**：
```toml
key1 = "value1"
key2 = "value2"

prompt = """
content here
with replacements
"""
```

---

## 完整範例

### 輸入 (example.md)
```markdown
---
title: Git Status Command
description: Show git repository status
author: DevTeam
version: 2.0
---

Please check $ARGUMENTS in the repository.

Use !`git status` to see changes.
Then use !`git diff` for details.
```

### 處理流程

1. **Parse**:
   - metadata = { "author": "DevTeam", "description": "Show...", "title": "Git...", "version": "2.0" }
   - content = "Please check $ARGUMENTS...\n\nUse !`git status`..."

2. **Validate**:
   - ✅ title 存在
   - ✅ description 存在
   - 通過驗證

3. **Replace**:
   - `$ARGUMENTS` → `{{ args }}`
   - `` !`git status` `` → `!{git status}`
   - `` !`git diff` `` → `!{git diff}`

4. **Generate**:
   ```toml
   author = "DevTeam"
   description = "Show git repository status"
   title = "Git Status Command"
   version = "2.0"

   prompt = """
   Please check {{ args }} in the repository.

   Use !{git status} to see changes.
   Then use !{git diff} for details.
   """
   ```

---

## 錯誤處理範例

### 缺少必要欄位

```markdown
---
title: Example
author: User
---

Content
```

**錯誤**：
```
Error: Invalid frontmatter format: This is not a markdown file for cc slash command.
```

**原因**：缺少 `description` 欄位。

---

## 測試覆蓋

### 單元測試（62 個）
- Parser: 12 tests (metadata + content + validation)
- Replacer: 14 tests ($ARGUMENTS + backticks)
- Generator: 9 tests (TOML output)
- Error: 5 tests (validation failures)
- Directory: 16 tests (path detection + file collection + prefix filtering)
  - 8 個前綴過濾測試：
    - ✅ 忽略 `~` 開頭的檔案
    - ✅ 忽略 `.` 開頭的檔案
    - ✅ 不忽略正常檔案（含中間有符號的檔名）
    - ✅ 忽略 `~` 開頭的資料夾
    - ✅ 忽略 `.` 開頭的資料夾
    - ✅ 不忽略正常資料夾
    - ✅ `collect_md_files` 忽略波浪符號和句點檔案
    - ✅ `collect_md_files` 忽略波浪符號和句點資料夾

### 整合測試（18 個）
- **資料夾轉換測試（8 個）**：
  - ✅ 巢狀目錄結構轉換
  - ✅ 驗證失敗處理
  - ✅ 空目錄錯誤處理
  - ✅ 部分成功情境
  - ✅ 深層嵌套支援
  - ✅ 路徑映射正確性
  - ✅ 特殊字元檔名
  - ✅ 前綴過濾（~ 和 .）
- **檔案轉換測試（10 個）**：
  - 6 個基本轉換測試（不驗證必要欄位）
  - 4 個驗證測試：
    - ✅ 通過驗證（有 title + description）
    - ❌ 缺少 description
    - ❌ 缺少 title
    - ❌ 沒有 metadata
