# 🚀 GitHub Actions Release Setup - 你需要做的操作

## 📋 **你需要修改的地方** (僅 3 個地方！)

### 1️⃣ **修改 Homebrew 用戶名**
**檔案**: `.github/workflows/release.yml`
**位置**: 第 151 行
```yaml
tap: monshakeys/tap  # ← 把 "monshakeys" 改成你的 GitHub 用戶名
```

**範例**:
- 如果你的 GitHub 用戶名是 `john-doe`，改成：`tap: john-doe/tap`

---

### 2️⃣ **修改 Release 說明** (可選)
**檔案**: `.github/workflows/release.yml`
**位置**: 第 160-180 行
```yaml
body: |
  ## Changes in this release

  # ← 在這裡添加你的版本說明
```

---

### 3️⃣ **修改專案名稱** (如果需要)
**檔案**: `.github/workflows/release.yml`
**位置**: 第 44-47 行
```yaml
binary_name: md2toml  # ← 改成你的專案名稱
asset_name: md2toml-aarch64-apple-darwin.tar.gz  # ← 改成你的專案名稱
```

---

## 🎯 **完整平台支援**

| 平台 | 架構 | 編譯狀態 | 檔案格式 |
|------|------|----------|----------|
| **Apple Silicon Mac** | arm64 | ✅ | `.tar.gz` |
| **Intel Mac** | x86_64 | ✅ | `.tar.gz` |
| **Linux x64** | x86_64 | ✅ | `.tar.gz` |
| **Windows x64** | x86_64 | ✅ | `.zip` |
| **Universal Mac** | arm64 + x86_64 | ✅ | `.tar.gz` |

**手機版本** (不建議)：
- Android ARM64 - 可添加 (需要 NDK)
- iOS ARM64 - 可添加 (需要 Xcode)

---

## 🎮 **使用方法**

### 方法 1: 自動觸發 (推薦)
```bash
git tag v1.0.0
git push origin v1.0.0
```

### 方法 2: 手動觸發
1. GitHub Repository > Actions
2. 選擇 "Release"
3. 點擊 "Run workflow"

---

## 📦 **生成的文件**

```
GitHub Release 會包含：
✅ md2toml-universal-apple-darwin.tar.gz  (Mac 通用版)
✅ md2toml-aarch64-apple-darwin.tar.gz    (Apple Silicon)
✅ md2toml-x86_64-apple-darwin.tar.gz     (Intel Mac)
✅ md2toml-x86_64-unknown-linux-gnu.tar.gz (Linux)
✅ md2toml-x86_64-pc-windows-msvc.zip      (Windows)
```

---

## 🔧 **工作流程**

```
1. 你推送版本標籤
       ↓
2. GitHub Actions 自動開始編譯 (4個平台並行)
       ↓
3. 創建 Universal Binary (Intel + Apple Silicon 合併)
       ↓
4. 打包並上傳到 GitHub Releases
       ↓
5. 發布到 Homebrew (如果已設置)
       ↓
6. 完成！用戶可以下載和使用
```

---

## 📚 **詳細文檔**

- 📖 `.github/workflows/README_RELEASE.md` - 完整說明
- 📱 `.github/workflows/MOBILE_TARGETS.md` - 手機版本指南

---

## ✅ **檢查清單**

在推送代碼前，確認：
- [ ] 已修改 Homebrew 用戶名 (第 151 行)
- [ ] 已確認專案名稱正確 (第 44-47 行)
- [ ] 已測試本地編譯：`cargo build --release`
- [ ] 已推送代碼到 GitHub

---

## 🎯 **結論**

**你只需要修改 1 行代碼** (Homebrew 用戶名)，其他都已經配置好了！

推送 `v1.0.0` 標籤後，約 5-10 分鐘就會自動生成所有平台的版本！ 🚀
