# 临时文件夹创建工具 (TemporaryFolderCreator)

这是一个用Rust编写的Windows工具，用于自动创建和管理临时文件夹。该工具可以按年月日的层级结构创建文件夹，并支持自动清理过期文件夹的功能。

## 功能特点

- 自动创建分层的临时文件夹（年月/日）
- 自动打开新创建的文件夹
- 支持配置基础路径
- 支持自动清理过期文件夹
- 通过setting.json进行配置

## 使用方法

1. 首次运行时会自动创建`setting.json`配置文件
2. 可以修改`setting.json`中的配置项：
   ```json
   {
     "base_path": "D:\\Temporary",  // 临时文件夹的基础路径
     "dayout": -1                   // 清理多少天前的文件，-1表示不自动清理，0表示每次都清理
   }
   ```
3. 运行程序后会自动创建格式为`YY/MM/DD`的文件夹结构
4. 程序会自动打开新创建的文件夹

## 配置说明

- `base_path`: 指定临时文件夹的根目录路径
- `dayout`: 设置自动清理的天数
  - 值为正数时：自动删除超过指定天数的文件夹
  - 值为-1时：禁用自动清理功能
  - 值为0时：每次都清理

## 系统要求

- Windows操作系统
- Rust运行环境

## 构建方法

```bash
cargo build --release
```

编译后的可执行文件位于`target/release/TemporaryFolderCreator.exe`。

## License

MIT
