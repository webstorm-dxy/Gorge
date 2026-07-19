//! 谱面包模型（对应 C# `Chart/Package.cs`）。
//!
//! 谱面包是谱面数据的容器，支持文件夹和 zip 两种格式。
//! 文件分类规则：`.g` 扩展名文件为源码，其余为资源文件。

use std::fs;
use std::io::{Read, Write};
use std::path::Path;

/// 源码文件（对应 C# `SourceCodeFile`）。
///
/// 包含 Gorge 语言源码，可能带有 UTF-8 BOM。
#[derive(Debug, Clone, PartialEq)]
pub struct SourceCodeFile {
    /// 文件相对路径（使用 `/` 分隔符）
    pub path: String,
    /// 源码内容（已剥离 BOM）
    pub code: String,
    /// 是否为谱面源码（true = 谱面，false = 模态）
    pub is_chart_source_code: bool,
}

impl SourceCodeFile {
    pub fn new(path: String, code: String, is_chart_source_code: bool) -> Self {
        Self { path, code, is_chart_source_code }
    }
}

/// 资源文件（对应 C# `AssetFile`）。
#[derive(Debug, Clone)]
pub struct AssetFile {
    /// 文件相对路径（使用 `/` 分隔符）
    pub path: String,
    /// 文件二进制数据
    pub data: Vec<u8>,
    /// 是否为谱面资源
    pub is_chart_asset: bool,
}

impl AssetFile {
    pub fn new(path: String, data: Vec<u8>, is_chart_asset: bool) -> Self {
        Self { path, data, is_chart_asset }
    }

    /// 深拷贝
    pub fn deep_copy(&self) -> Self {
        Self {
            path: self.path.clone(),
            data: self.data.clone(),
            is_chart_asset: self.is_chart_asset,
        }
    }
}

/// 谱面包（对应 C# `Package`）。
///
/// 谱面的文件级容器，可从文件夹或 zip 加载，也可保存为 zip。
#[derive(Debug, Clone, Default)]
pub struct Package {
    /// 资源文件列表
    pub asset_files: Vec<AssetFile>,
    /// 源码文件列表
    pub source_code_files: Vec<SourceCodeFile>,
}

/// 包操作错误类型
#[derive(Debug)]
pub enum PackageError {
    IoError(std::io::Error),
    ZipError(String),
    FolderNotFound(String),
}

impl std::fmt::Display for PackageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PackageError::IoError(e) => write!(f, "IO 错误: {}", e),
            PackageError::ZipError(e) => write!(f, "Zip 错误: {}", e),
            PackageError::FolderNotFound(path) => write!(f, "目标文件夹不存在: {}", path),
        }
    }
}

impl std::error::Error for PackageError {}

impl From<std::io::Error> for PackageError {
    fn from(e: std::io::Error) -> Self {
        PackageError::IoError(e)
    }
}

impl Package {
    /// 创建空谱面包
    pub fn new() -> Self {
        Self {
            asset_files: Vec::new(),
            source_code_files: Vec::new(),
        }
    }

    /// 从文件夹加载包（同步，对应 C# `LoadFolderPackage`）。
    ///
    /// 遍历文件夹内所有文件：
    /// - `.g` 扩展名 → `SourceCodeFile`（自动剥离 UTF-8 BOM）
    /// - 其余 → `AssetFile`
    ///
    /// C# 的异步变体（`LoadFolderPackageAsync`）是 Unity 主线程需求，Rust 侧仅提供同步版本。
    pub fn load_folder_package<P: AsRef<Path>>(folder_path: P, is_chart: bool) -> Result<Self, PackageError> {
        let folder = folder_path.as_ref();
        if !folder.is_dir() {
            return Err(PackageError::FolderNotFound(folder.display().to_string()));
        }

        let mut package = Package::new();
        Self::walk_folder(folder, folder, is_chart, &mut package)?;
        Ok(package)
    }

    /// 递归遍历文件夹
    fn walk_folder(
        base_path: &Path,
        current_path: &Path,
        is_chart: bool,
        package: &mut Package,
    ) -> Result<(), PackageError> {
        for entry in fs::read_dir(current_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                Self::walk_folder(base_path, &path, is_chart, package)?;
            } else {
                let relative = path
                    .strip_prefix(base_path)
                    .unwrap_or(&path)
                    .to_string_lossy()
                    .replace('\\', "/");
                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");

                if file_name.ends_with(".g") {
                    let raw_bytes = fs::read(&path)?;
                    let code = strip_utf8_bom(&raw_bytes);
                    package.source_code_files.push(SourceCodeFile::new(
                        relative,
                        code,
                        is_chart,
                    ));
                } else {
                    let data = fs::read(&path)?;
                    package.asset_files.push(AssetFile::new(relative, data, is_chart));
                }
            }
        }
        Ok(())
    }

    /// 从 zip 文件路径加载包（对应 C# `LoadZipPackage(string, bool)`）。
    pub fn load_zip_package<P: AsRef<Path>>(zip_path: P, is_chart: bool) -> Result<Self, PackageError> {
        let file = fs::File::open(zip_path.as_ref())?;
        Self::load_zip_from_reader(file, is_chart)
    }

    /// 从 zip 字节数据加载包（对应 C# `LoadZipPackage(byte[], bool)`）。
    pub fn load_zip_from_bytes(data: &[u8], is_chart: bool) -> Result<Self, PackageError> {
        let cursor = std::io::Cursor::new(data);
        Self::load_zip_from_reader(cursor, is_chart)
    }

    /// 从实现了 Read + Seek 的 reader 加载 zip 包
    fn load_zip_from_reader<R: Read + std::io::Seek>(reader: R, is_chart: bool) -> Result<Self, PackageError> {
        let mut archive = zip::ZipArchive::new(reader)
            .map_err(|e| PackageError::ZipError(e.to_string()))?;
        let mut package = Package::new();

        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)
                .map_err(|e| PackageError::ZipError(e.to_string()))?;
            let entry_name = entry.name().to_string();

            // 跳过目录条目
            if entry.is_dir() {
                continue;
            }

            let mut buf = Vec::new();
            entry.read_to_end(&mut buf)
                .map_err(|e| PackageError::IoError(e))?;

            if entry_name.ends_with(".g") {
                let code = strip_utf8_bom(&buf);
                package.source_code_files.push(SourceCodeFile::new(
                    entry_name,
                    code,
                    is_chart,
                ));
            } else {
                package.asset_files.push(AssetFile::new(entry_name, buf, is_chart));
            }
        }
        Ok(package)
    }

    /// 保存为 zip 包（对应 C# `SaveZipPackage`）。
    ///
    /// 仅保存标记为谱面的文件（`is_chart_source_code` / `is_chart_asset`）。
    pub fn save_zip_package<P: AsRef<Path>>(&self, zip_path: P) -> Result<(), PackageError> {
        let file = fs::File::create(zip_path.as_ref())?;
        let mut writer = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        for source_file in &self.source_code_files {
            if !source_file.is_chart_source_code {
                continue;
            }
            writer
                .start_file(&source_file.path, options)
                .map_err(|e| PackageError::ZipError(e.to_string()))?;
            writer
                .write_all(source_file.code.as_bytes())
                .map_err(|e| PackageError::IoError(e))?;
        }

        for asset_file in &self.asset_files {
            if !asset_file.is_chart_asset {
                continue;
            }
            writer
                .start_file(&asset_file.path, options)
                .map_err(|e| PackageError::ZipError(e.to_string()))?;
            writer
                .write_all(&asset_file.data)
                .map_err(|e| PackageError::IoError(e))?;
        }

        writer.finish().map_err(|e| PackageError::ZipError(e.to_string()))?;
        Ok(())
    }
}

/// 剥离 UTF-8 BOM（对应 C# 中 `data[0]==0xEF && data[1]==0xBB && data[2]==0xBF` 检测）。
///
/// 若数据前三字节为 BOM 标记，则跳过并解码剩余部分；否则直接解码。
fn strip_utf8_bom(data: &[u8]) -> String {
    if data.len() >= 3 && data[0] == 0xEF && data[1] == 0xBB && data[2] == 0xBF {
        String::from_utf8_lossy(&data[3..]).into_owned()
    } else {
        String::from_utf8_lossy(data).into_owned()
    }
}

/// 检测是否为 zip 文件（魔数 PK\03\04）
pub fn is_zip_file(data: &[u8]) -> bool {
    data.len() >= 4 && data[0] == 0x50 && data[1] == 0x4B && data[2] == 0x03 && data[3] == 0x04
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_utf8_bom_with_bom() {
        let data = vec![0xEF, 0xBB, 0xBF, b'H', b'e', b'l', b'l', b'o'];
        let result = strip_utf8_bom(&data);
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_strip_utf8_bom_without_bom() {
        let data = b"Hello World";
        let result = strip_utf8_bom(data);
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_strip_utf8_bom_empty() {
        let result = strip_utf8_bom(&[]);
        assert_eq!(result, "");
    }

    #[test]
    fn test_strip_utf8_bom_only_bom() {
        let data = vec![0xEF, 0xBB, 0xBF];
        let result = strip_utf8_bom(&data);
        assert_eq!(result, "");
    }

    #[test]
    fn test_load_folder_package() {
        let temp_dir = std::env::temp_dir().join("gorge_test_package_load");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        // 创建源码文件（带 BOM）
        let bom_code = vec![0xEF, 0xBB, 0xBF, b'c', b'l', b'a', b's', b's', b' ', b'T'];
        fs::write(temp_dir.join("test.g"), &bom_code).unwrap();

        // 创建无 BOM 源码文件
        fs::write(temp_dir.join("test2.g"), b"class Test2 {}").unwrap();

        // 创建资源文件
        fs::write(temp_dir.join("image.png"), b"fake_png_data").unwrap();

        // 创建子目录中的文件
        fs::create_dir_all(temp_dir.join("sub")).unwrap();
        fs::write(temp_dir.join("sub").join("sub.g"), b"class Sub {}").unwrap();

        let package = Package::load_folder_package(&temp_dir, true).unwrap();

        assert_eq!(package.source_code_files.len(), 3);
        assert_eq!(package.asset_files.len(), 1);

        // 验证 BOM 剥离
        let test_g = package.source_code_files.iter().find(|f| f.path == "test.g").unwrap();
        assert_eq!(test_g.code, "class T");
        assert!(test_g.is_chart_source_code);

        // 验证无 BOM
        let test2_g = package.source_code_files.iter().find(|f| f.path == "test2.g").unwrap();
        assert_eq!(test2_g.code, "class Test2 {}");

        // 验证子目录
        let sub_g = package.source_code_files.iter().find(|f| f.path == "sub/sub.g").unwrap();
        assert_eq!(sub_g.code, "class Sub {}");

        // 清理
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_load_folder_package_not_found() {
        let result = Package::load_folder_package("nonexistent_path_12345", true);
        assert!(result.is_err());
    }

    #[test]
    fn test_zip_roundtrip() {
        // 构造包
        let mut package = Package::new();
        package.source_code_files.push(SourceCodeFile::new(
            "chart.g".to_string(),
            "class Test {} class Test2 {}".to_string(),
            true,
        ));
        package.source_code_files.push(SourceCodeFile::new(
            "modal.g".to_string(),
            "class Modal {}".to_string(),
            false,
        ));
        package.asset_files.push(AssetFile::new(
            "image.png".to_string(),
            vec![1, 2, 3, 4],
            true,
        ));
        package.asset_files.push(AssetFile::new(
            "private.bin".to_string(),
            vec![5, 6],
            false,
        ));

        // 保存为 zip
        let zip_path = std::env::temp_dir().join("gorge_test_package.zip");
        package.save_zip_package(&zip_path).unwrap();

        // 重新加载
        let loaded = Package::load_zip_package(&zip_path, true).unwrap();

        // 仅谱面文件应被保存
        assert_eq!(loaded.source_code_files.len(), 1, "仅 chart.g（谱面源码）应被保存");
        assert_eq!(loaded.source_code_files[0].path, "chart.g");
        assert!(!loaded.source_code_files[0].code.is_empty());
        assert_eq!(loaded.asset_files.len(), 1, "仅 image.png（谱面资源）应被保存");

        let _ = fs::remove_file(&zip_path);
    }

    #[test]
    fn test_zip_roundtrip_in_memory() {
        let mut package = Package::new();
        package.source_code_files.push(SourceCodeFile::new(
            "test.g".to_string(),
            "class A {}".to_string(),
            true,
        ));
        package.asset_files.push(AssetFile::new(
            "data.bin".to_string(),
            vec![1, 2, 3],
            true,
        ));

        // 保存到内存
        let zip_path = std::env::temp_dir().join("gorge_test_mem.zip");
        package.save_zip_package(&zip_path).unwrap();
        let zip_bytes = fs::read(&zip_path).unwrap();
        let _ = fs::remove_file(&zip_path);

        // 从内存加载
        let loaded = Package::load_zip_from_bytes(&zip_bytes, true).unwrap();
        assert_eq!(loaded.source_code_files.len(), 1);
        assert_eq!(loaded.source_code_files[0].code, "class A {}");
        assert_eq!(loaded.asset_files.len(), 1);
        assert_eq!(loaded.asset_files[0].data, vec![1, 2, 3]);
    }

    #[test]
    fn test_is_zip_file() {
        // ZIP 魔数
        assert!(is_zip_file(&[0x50, 0x4B, 0x03, 0x04]));
        assert!(is_zip_file(&[0x50, 0x4B, 0x03, 0x04, 0x00, 0x00]));
        // 非 ZIP
        assert!(!is_zip_file(&[0x47, 0x4F, 0x52, 0x47])); // GORG
        assert!(!is_zip_file(&[]));
    }

    #[test]
    fn test_load_gorge_file_directory_smoke() {
        // 冒烟测试：加载 references/gorge_file/ 真实内容
        let gorge_file_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("references")
            .join("gorge_file");

        if gorge_file_dir.is_dir() {
            let package = Package::load_folder_package(&gorge_file_dir, true).unwrap();
            // 15 个文件：.g 文件为源码，.gorge 文件为资源
            assert!(package.source_code_files.len() >= 10, "应有至少 10 个 .g 源码文件");
            // .gorge 文件应被归类为资源（不以 .g 结尾）
            assert!(package.asset_files.len() >= 3, "应有至少 3 个 .gorge 资源文件");
        }
    }

    #[test]
    fn test_source_code_file_new() {
        let scf = SourceCodeFile::new("test.g".to_string(), "code".to_string(), true);
        assert_eq!(scf.path, "test.g");
        assert_eq!(scf.code, "code");
        assert!(scf.is_chart_source_code);
    }

    #[test]
    fn test_asset_file_deep_copy() {
        let af = AssetFile::new("test.png".to_string(), vec![1, 2, 3], true);
        let copy = af.deep_copy();
        assert_eq!(copy.path, "test.png");
        assert_eq!(copy.data, vec![1, 2, 3]);
        assert!(copy.is_chart_asset);
    }
}
