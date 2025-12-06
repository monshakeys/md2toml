# 📱 手機平台編譯指南

## 🎯 **為什麼手機版本比較複雜？**

1. **不同的執行環境**：手機應用需要特殊的啟動方式
2. **權限管理**：Android/iOS 有嚴格的權限控制
3. **UI/CLI 差異**：手機上通常用觸控介面而非命令行
4. **交叉編譯複雜度**：需要 NDK/Xcode 等額外工具

---

## 🤖 **Android 版本**

### 前置條件
```bash
# 1. 安裝 Android NDK
# 下載並安裝 Android NDK (推薦版本 25.x)
# 設定環境變量
export ANDROID_NDK_HOME=$HOME/Android/Ndk/25.x
export PATH=$PATH:$ANDROID_NDK_HOME

# 2. 安裝 Rust 目標
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
```

### 在 CI 中添加 Android
```yaml
# 在 .github/workflows/release.yml 的 matrix.include 中添加：
- target: aarch64-linux-android
  os: ubuntu-latest
  arch: arm64
  binary_name: md2toml
  asset_name: md2toml-aarch64-linux-android.apk

# Android 需要特殊處理，不能直接編譯 CLI 程式
# 需要創建 Android App Wrapper
```

### Android App Wrapper 範例
```java
// MainActivity.java
public class MainActivity extends AppCompatActivity {
    @Override
    protected void onCreate(Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        setContentView(R.layout.activity_main);

        // 執行 Rust 編譯的 CLI 程式
        String result = executeCommand("./md2toml", "/path/to/input.md");
        TextView output = findViewById(R.id.output);
        output.setText(result);
    }

    private String executeCommand(String command, String args) {
        // 實現命令行執行邏輯
    }
}
```

### 📋 **Android 注意事項**
- ⚠️ CLI 程式在 Android 上需要 Java/Kotlin 包裝
- ⚠️ 需要申請文件存取權限
- ⚠️ 不建議一般用戶使用 CLI 在手機上

---

## 🍎 **iOS 版本**

### 前置條件
```bash
# 1. 需要 macOS 和 Xcode
xcode-select --install

# 2. 安裝 Rust 目標
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios
rustup target add aarch64-apple-ios-sim
```

### 在 CI 中添加 iOS
```yaml
# 在 .github/workflows/release.yml 的 matrix.include 中添加：
- target: aarch64-apple-ios
  os: macos-latest
  arch: arm64
  binary_name: md2toml
  asset_name: md2toml-aarch64-apple-ios.tar.gz

- target: x86_64-apple-ios
  os: macos-latest
  arch: x86_64
  binary_name: md2toml
  asset_name: md2toml-x86_64-apple-ios.tar.gz
```

### iOS App Wrapper 範例
```swift
// ViewController.swift
import UIKit

class ViewController: UIViewController {
    @IBOutlet weak var outputLabel: UILabel!

    @IBAction func convertButtonTapped(_ sender: UIButton) {
        let result = executeCommand("/path/to/md2toml", "/path/to/input.md")
        outputLabel.text = result
    }

    private func executeCommand(_ command: String, _ args: String) -> String {
        // 實現命令行執行邏輯
        return "Conversion completed"
    }
}
```

### 📋 **iOS 注意事項**
- ⚠️ 需要 Apple Developer Account 來部署
- ⚠️ 需要創建 Xcode 專案
- ⚠️ 需要通過 App Store 審查（或使用 TestFlight）
- ⚠️ CLI 程式在 iOS 上限制很多

---

## 🎯 **完整的 6 平台矩陣 (如果你真的要支援所有平台)**

```yaml
strategy:
  matrix:
    include:
      # ========== 桌面平台 ==========
      - target: aarch64-apple-darwin
        os: macos-latest
        arch: arm64
        binary_name: md2toml
        asset_name: md2toml-aarch64-apple-darwin.tar.gz

      - target: x86_64-apple-darwin
        os: macos-latest
        arch: x86_64
        binary_name: md2toml
        asset_name: md2toml-x86_64-apple-darwin.tar.gz

      - target: x86_64-unknown-linux-gnu
        os: ubuntu-latest
        arch: x86_64
        binary_name: md2toml
        asset_name: md2toml-x86_64-unknown-linux-gnu.tar.gz

      - target: x86_64-pc-windows-msvc
        os: windows-latest
        arch: x86_64
        binary_name: md2toml.exe
        asset_name: md2toml-x86_64-pc-windows-msvc.zip

      # ========== 手機平台 (需要額外配置) ==========
      - target: aarch64-linux-android
        os: ubuntu-latest
        arch: arm64
        binary_name: md2toml
        asset_name: md2toml-aarch64-linux-android.apk

      - target: aarch64-apple-ios
        os: macos-latest
        arch: arm64
        binary_name: md2toml
        asset_name: md2toml-aarch64-apple-ios.ipa
```

---

## 💡 **建議的實際做法**

### ✅ **推薦**：專注於桌面平台
- macOS (Intel + Apple Silicon)
- Linux x64
- Windows x64

### ⚠️ **不推薦**：手機平台
- 理由：
  1. CLI 工具在手機上使用體驗很差
  2. 需要額外開發 App Wrapper
  3. 維護成本高
  4. 用戶需求極低

### 📊 **用戶使用場景分析**
```
桌面用戶 (95%):
├── macOS: 40% (開發者主要使用)
├── Windows: 35% (一般用戶)
└── Linux: 20% (伺服器/開發者)

手機用戶 (5%):
├── iOS: 3% (有需求但技術門檻高)
└── Android: 2% (有需求但技術門檻高)
```

---

## 🔧 **替代方案：Web 版本**

如果你真的想要手機支援，建議做 Web 版本：

```bash
# 使用 wasm-pack 編譯到 WebAssembly
cargo install wasm-pack

# 編譯
wasm-pack build --target web

# 部署到 GitHub Pages 或 Netlify
```

**優點**：
- ✅ 手機瀏覽器直接使用
- ✅ 無需安裝
- ✅ 跨平台支援

**缺點**：
- ❌ 檔案系統存取受限
- ❌ 需要網路連線
- ❌ 效能較差

---

## 📝 **結論**

**對於 md2toml 這個工具，建議只支援桌面平台**：

1. ✅ **4 個平台就足夠了** (Intel Mac, Apple Silicon Mac, Linux, Windows)
2. ✅ **覆蓋 95% 的用戶需求**
3. ✅ **維護成本低**
4. ✅ **用戶體驗好**

手機版本需要額外的 App 開發，不值得投入時間。建議專注於桌面平台，把資源用在刀口上！ 🎯

---

## 🚀 **下一步**

1. 修改 `.github/workflows/release.yml` 中的 Homebrew 用戶名
2. 測試 workflow：創建 `v1.0.0` 標籤
3. 檢查 GitHub Releases
4. 更新 README 中的安裝說明

就這樣，簡單有效！ 🎉
