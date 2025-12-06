# 🚀 GitHub Actions Release Workflow 說明

## 📋 這個 workflow 做了什麼？

這個 workflow 會自動編譯你的 Rust 程式到 **4 個主要平台**：
1. ✅ **Apple Silicon Mac** (aarch64-apple-darwin) - 你目前的環境
2. ✅ **Intel Mac** (x86_64-apple-darwin)
3. ✅ **Linux x64** (x86_64-unknown-linux-gnu)
4. ✅ **Windows x64** (x86_64-pc-windows-msvc)

並且會：
- 📦 自動打包成壓縮檔
- 🔗 創建 GitHub Release
- 🍺 發布到 Homebrew (可選)

---

## 🎯 **你需要添加的內容** (必須修改！)

### 1️⃣ **修改 Homebrew 用戶名**
```yaml
# 檔案: .github/workflows/release.yml
# 找到這一行並修改：
tap: monshakeys/tap  # ← 把 "monshakeys" 改成你的 GitHub 用戶名
```

### 2️⃣ **修改 Release 說明**
```yaml
# 檔案: .github/workflows/release.yml
# 找到 "Create Release" 部分，修改這個區塊：
body: |
  ## Changes in this release

  # ← 改成你的專案說明
```

### 3️⃣ **修改檔案名稱 (如果需要)**
```yaml
# 如果你的專案名稱不是 "md2toml"，需要修改：
binary_name: md2toml  # ← 改成你的專案名稱
asset_name: md2toml-aarch64-apple-darwin.tar.gz  # ← 改成你的專案名稱
```

---

## 🎮 **如何使用這個 workflow**

### 方法 1: 自動觸發 (推薦)
```bash
# 創建一個版本標籤
git tag v1.0.0
git push origin v1.0.0
```

### 方法 2: 手動觸發
1. 進入 GitHub Repository
2. 點擊 "Actions" 標籤
3. 選擇 "Release" workflow
4. 點擊 "Run workflow" 按鈕

---

## 📱 **手機版本編譯 (進階)**

如果你真的想要手機版本，這裡是方法：

### Android
```bash
# 安裝 Android NDK
rustup target add aarch64-linux-android
cargo build --release --target aarch64-linux-android

# 需要在 CI 中添加：
- target: aarch64-linux-android
  os: ubuntu-latest
  arch: arm64
  binary_name: md2toml
  asset_name: md2toml-aarch64-linux-android.tar.gz
```

### iOS
```bash
# 需要 Xcode 和 Apple Developer Account
rustup target add aarch64-apple-ios
cargo build --release --target aarch64-apple-ios

# 需要在 CI 中添加：
- target: aarch64-apple-ios
  os: macos-latest
  arch: arm64
  binary_name: md2toml
  asset_name: md2toml-aarch64-apple-ios.tar.gz
```

**⚠️ 注意**：手機版本需要額外配置，不建議一般用戶使用。

---

## 🔧 **CI 完整平台支援矩陣**

| 平台 | 架構 | 狀態 | 安裝方式 |
|------|------|------|----------|
| **macOS (Apple Silicon)** | arm64 | ✅ 支援 | `brew install monshakeys/tap/md2toml` |
| **macOS (Intel)** | x86_64 | ✅ 支援 | `brew install monshakeys/tap/md2toml` |
| **Linux x64** | x86_64 | ✅ 支援 | 下載 .tar.gz |
| **Windows x64** | x86_64 | ✅ 支援 | 下載 .zip |
| **Android** | arm64 | ⚠️ 可添加 | 需要 NDK |
| **iOS** | arm64 | ⚠️ 可添加 | 需要 Xcode |

---

## 🚨 **重要的 CI 設置**

### 1. 確保 Cross 安裝
workflow 中會自動安裝 `cross` 工具，用於交叉編譯。

### 2. 標籤命名規則
```
✅ v1.0.0     (觸發 release)
✅ v0.1.0     (觸發 release)
❌ 1.0.0      (不會觸發)
❌ release    (不會觸發)
```

### 3. 權限設置
在 GitHub Repository Settings > Actions > General 中：
- ✅ Allow GitHub Actions to create and approve pull requests
- ✅ Allow actions and reusable workflows to read repository metadata

---

## 🎁 **生成的文件結構**

```bash
# GitHub Release 中會包含：
md2toml-universal-apple-darwin.tar.gz    # Mac 通用版 (Intel + Apple Silicon)
md2toml-aarch64-apple-darwin.tar.gz      # Apple Silicon Mac
md2toml-x86_64-apple-darwin.tar.gz       # Intel Mac
md2toml-x86_64-unknown-linux-gnu.tar.gz  # Linux
md2toml-x86_64-pc-windows-msvc.zip       # Windows
```

---

## 🔍 **故障排除**

### 問題 1: 編譯失敗
```bash
# 解決方案：
# 1. 檢查 Cargo.toml 是否有不支持的依賴
# 2. 確認所有 target 都已安裝
rustup target list --installed
```

### 問題 2: Homebrew 發布失敗
```bash
# 解決方案：
# 1. 確認你已經在 GitHub 創建了 Homebrew tap
# 2. 檢查 GITHUB_TOKEN 權限
```

### 問題 3: Universal Binary 創建失敗
```bash
# 解決方案：
# 1. 確認兩個 Mac 版本都成功編譯
# 2. 檢查 lipo 工具是否可用
```

---

## 📝 **建議的工作流程**

```bash
# 1. 開發完成後
git add .
git commit -m "feat: add new feature"
git push origin main

# 2. 創建版本標籤 (觸發 CI)
git tag v1.0.0
git push origin v1.0.0

# 3. 等待 GitHub Actions 完成 (約 5-10 分鐘)
# 4. 檢查 GitHub Releases 頁面
# 5. 更新 README 或 Documentation
```

---

## 🎯 **總結**

這個 workflow 會自動為你：
1. ✅ 編譯 4 個平台的版本
2. ✅ 創建 Universal Binary (Intel + Apple Silicon)
3. ✅ 打包並上傳到 GitHub Releases
4. ✅ 發布到 Homebrew (如果你有 tap)

**你需要做的**：修改 Homebrew 用戶名和 Release 說明！

其他部分都已經配置好了，直接使用即可！ 🚀
