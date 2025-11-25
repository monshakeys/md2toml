# md2toml 專案結構說明

## 🎯 專案功能概述


輸入：
```markdown
---
title: Hello
author: User
---

Process $ARGUMENTS with !`ls` command
```

輸出：
```toml
author = "User"
title = "Hello"

prompt = """
Process {{ args }} with !{ls} command
"""
```

---

## 📁 專案檔案結構（繁中詳解）

```
md2toml/
│
├── 📦 Cargo.toml
├── 🔒 Cargo.lock
├── 📂 src/
│   ├── main.rs            # 🚪 程式入口點
│   ├── lib.rs             # 📖 函式庫總目錄
│   ├── parser.rs          # 🔍 解析器模組
│   ├── replacer.rs        # 🔄 替換器模組
│   └── generator.rs       # 🏭 生成器模組
├── 📂 tests/              # 測試資料夾
└── 📂 fixtures/
    ├── simple.md
    ├── simple.expected.toml
    ├── complex.md
    └── complex.expected.toml
```

---

## 🔧 各模組功能詳解（用 Django 比喻）

### 1. `main.rs` - 程式入口（manage.py）

```rust
// 就像 Django 的 manage.py runserver
// 這裡處理命令列參數

fn main() {
    // 1. 讀取使用者輸入的檔案路徑（像解析 request）
    let args = parse_args();

    // 2. 讀取檔案內容
    let content = read_file(args.input);

    // 3. 呼叫其他模組處理（像 Django 的 view 呼叫 model 和 serializer）
    let metadata = parse_frontmatter(&content);  // parser.rs
    let body = extract_content(&content);         // parser.rs
    let processed = apply_all_replacements(&body); // replacer.rs
    let toml = generate_toml(metadata, processed); // generator.rs

    // 4. 寫入輸出檔案
    write_file(args.output, toml);
}
```

### 2. `parser.rs` - 解析器（forms.py / serializers.py）

```rust
// 就像 Django 的 Form 或 Serializer 驗證並提取資料

// 提取 YAML 前置資料（類似 form.cleaned_data）
pub fn parse_frontmatter(content: &str) -> HashMap<String, String> {
    // 把 ---\ntitle: Test\n--- 轉成 {"title": "Test"}
}

// 提取內容（類似取得 request.body）
pub fn extract_content(markdown: &str) -> String {
    // 把 YAML 後面的內容全部取出來
    // 並保持原始格式（換行、縮排）
}
```

**Django 比喻：**
```python
# Django Form
class MarkdownForm(forms.Form):
    title = forms.CharField()
    author = forms.CharField()

    def parse_frontmatter(self):
        return self.cleaned_data  # 類似我們的 parse_frontmatter
```

### 3. `replacer.rs` - 替換器（template filters）

```rust
// 就像 Django 模板的 filter：{{ value|upper }}

// $ARGUMENTS → {{ args }}（類似 Django filter）
pub fn replace_arguments(content: &str) -> String {
    content.replace("$ARGUMENTS", "{{ args }}")
}

// !`command` → !{command}（另一個 filter）
pub fn replace_backtick_patterns(content: &str) -> String {
    // 用正則表達式替換
    regex.replace_all(content, "!{$1}")
}

// 套用所有替換（像 Django 連續用多個 filter）
pub fn apply_all_replacements(content: &str) -> String {
    let step1 = replace_arguments(content);
    let step2 = replace_backtick_patterns(&step1);
    step2
}
```

**Django 比喻：**
```python
# Django template
{{ content|replace_arguments|replace_backticks }}

# 或在 view 中
def process_content(content):
    content = replace_arguments(content)
    content = replace_backticks(content)
    return content
```

### 4. `generator.rs` - 生成器（render template）

```rust
// 就像 Django 的 render() 把資料塞進模板

pub fn generate_toml(metadata: HashMap, prompt: String) -> String {
    let mut output = String::new();

    // 生成 TOML 格式（類似填充模板）
    // title = "Test"
    // author = "User"
    //
    // prompt = """
    // content here
    // """

    output
}
```

**Django 比喻：**
```python
# Django view
def render_toml(metadata, prompt):
    return render(request, 'output.toml', {
        'metadata': metadata,
        'prompt': prompt
    })
```

### 5. `lib.rs` - 函式庫入口（__init__.py）

```rust
// 就像 Django app 的 __init__.py
// 把各模組的功能「公開」出去

pub mod parser;      // 引入 parser 模組
pub mod replacer;    // 引入 replacer 模組
pub mod generator;   // 引入 generator 模組

// 重新匯出（讓外界可以直接用）
pub use parser::{parse_frontmatter, extract_content};
pub use replacer::apply_all_replacements;
pub use generator::generate_toml;
```

**Django 比喻：**
```python
# myapp/__init__.py
from .views import MyView
from .models import MyModel
from .serializers import MySerializer

# 這樣外界就可以：from myapp import MyView
```

---

## 🧪 測試架構（Django TestCase）

### 單元測試（在各模組內）

```rust
// 在 parser.rs 檔案內
#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_frontmatter() {
        // 測試單一功能
        let input = "---\ntitle: Test\n---";
        let result = parse_frontmatter(input);
        assert_eq!(result.get("title"), Some(&"Test".to_string()));
    }
}
```

**Django 比喻：**
```python
# tests.py
class ParserTestCase(TestCase):
    def test_parse_frontmatter(self):
        input_data = "---\ntitle: Test\n---"
        result = parse_frontmatter(input_data)
        self.assertEqual(result['title'], 'Test')
```

### 整合測試（tests/integration_tests.rs）

```rust
// 測試完整流程
#[test]
fn test_integration_simple() {
    let input = read_fixture("simple.md");
    let expected = read_fixture("simple.expected.toml");

    // 完整轉換流程
    let result = convert_md_to_toml(&input);

    assert_eq!(result, expected);
}
```

**Django 比喻：**
```python
# tests.py
class IntegrationTestCase(TestCase):
    def test_full_conversion(self):
        # 測試完整的 view 流程
        response = self.client.post('/convert/', {
            'file': open('simple.md')
        })
        self.assertEqual(response.content, expected_output)
```


### 執行檔

```bash
# 編譯 release 版本（優化過的）
cargo build --release

# 執行檔在這裡：
./target/release/md2toml
```

### 4. 使用命令列工具

```bash
# 基本用法
./target/release/md2toml input.md

# 指定輸出檔案
./target/release/md2toml input.md -o output.toml

# 查看幫助
./target/release/md2toml --help
```

