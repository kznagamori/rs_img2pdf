use crate::error::{AppError, Result};
use image::{DynamicImage, GenericImageView, ImageOutputFormat};
use log::{debug, info, warn};
use pdf_writer::{Pdf, Ref, Name, Rect, Filter, Finish};
use std::collections::BTreeMap;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// 画像ファイルをPDFに変換するコンバーター
pub struct ImageToPdfConverter {
    /// サポートされている画像拡張子
    supported_extensions: Vec<&'static str>,
}

impl ImageToPdfConverter {
    /// 新しいコンバーターインスタンスを作成する
    ///
    /// # Returns
    /// * `Self` - コンバーターインスタンス
    pub fn new() -> Self {
        Self {
            supported_extensions: vec!["jpg", "jpeg", "png", "webp"],
        }
    }

    /// 指定されたディレクトリの画像ファイルをPDFに変換する
    ///
    /// # Arguments
    /// * `input_dir` - 入力ディレクトリのパス
    /// * `output_file` - 出力PDFファイルのパス
    ///
    /// # Returns
    /// * `Result<()>` - 変換の成功/失敗
    pub fn convert(&self, input_dir: &Path, output_file: &Path) -> Result<()> {
        info!("Starting image to PDF conversion");
        
        // 画像ファイルを収集してソート
        let image_files = self.collect_and_sort_images(input_dir)?;
        
        if image_files.is_empty() {
            return Err(AppError::NoImagesFound(input_dir.to_string_lossy().to_string()));
        }

        info!("Found {} image files", image_files.len());

        // PDFを作成
        let mut pdf = Pdf::new();
        
        // カタログとページツリーのID
        let catalog_id = Ref::new(1);
        let page_tree_id = Ref::new(2);
        
        let mut page_ids = Vec::new();
        let mut next_id = 3;
        
        // 各画像ファイルを処理
        for (index, file_path) in image_files.iter().enumerate() {
            info!("Processing image {}/{}: {:?}", index + 1, image_files.len(), file_path);
            
            // 画像を読み込んで処理
            let processed_image = self.process_image(file_path)?;
            
            // 画像をPDFページに追加
            let page_id = Ref::new(next_id);
            next_id += 1;
            
            self.add_image_as_page(&mut pdf, page_id, &processed_image, &mut next_id)?;
            page_ids.push(page_id);
        }

        // ページツリーを作成
        let mut page_tree = pdf.pages(page_tree_id);
        page_tree.kids(page_ids.iter().copied());
        page_tree.count(page_ids.len() as i32);
        page_tree.finish();

        // カタログを作成
        let mut catalog = pdf.catalog(catalog_id);
        catalog.pages(page_tree_id);
        catalog.finish();

        // PDFファイルを保存
        info!("Saving PDF file: {:?}", output_file);
        let bytes = pdf.finish();
        fs::write(output_file, bytes)?;
        
        info!("PDF saved successfully: {:?}", output_file);
        Ok(())
    }

    /// 指定されたディレクトリから画像ファイルを収集し、ファイル名でソートする
    ///
    /// # Arguments
    /// * `dir` - 検索するディレクトリ
    ///
    /// # Returns
    /// * `Result<Vec<PathBuf>>` - ソートされた画像ファイルのリスト
    fn collect_and_sort_images(&self, dir: &Path) -> Result<Vec<PathBuf>> {
        let mut image_files = BTreeMap::new();

        // ディレクトリを再帰的に検索
        for entry in WalkDir::new(dir).max_depth(1).into_iter() {
            let entry = entry?;
            let path = entry.path();
            
            // ファイルかどうかチェック
            if !path.is_file() {
                continue;
            }

            // 拡張子をチェック
            if let Some(extension) = path.extension() {
                let ext_str = extension.to_string_lossy().to_lowercase();
                if self.supported_extensions.contains(&ext_str.as_str()) {
                    // ファイル名から数値を抽出してソートキーとして使用
                    if let Some(sort_key) = self.extract_numeric_sort_key(path) {
                        image_files.insert(sort_key, path.to_path_buf());
                    } else {
                        // 数値が抽出できない場合は、ファイル名をそのまま使用
                        warn!("Could not extract numeric sort key from: {:?}", path);
                        let file_name = path.file_name()
                            .and_then(|name| name.to_str())
                            .unwrap_or("")
                            .to_string();
                        image_files.insert(file_name, path.to_path_buf());
                    }
                }
            }
        }

        let sorted_files: Vec<PathBuf> = image_files.into_values().collect();
        debug!("Collected {} image files", sorted_files.len());
        
        Ok(sorted_files)
    }

    /// ファイル名から数値のソートキーを抽出する
    ///
    /// # Arguments
    /// * `path` - ファイルパス
    ///
    /// # Returns
    /// * `Option<String>` - ソートキー（数値を0パディングした文字列）
    fn extract_numeric_sort_key(&self, path: &Path) -> Option<String> {
        let file_stem = path.file_stem()?.to_str()?;
        
        // 数値部分を抽出
        let numeric_part: String = file_stem.chars().filter(|c| c.is_ascii_digit()).collect();
        
        if numeric_part.is_empty() {
            return None;
        }

        // 数値を整数に変換してから、0パディングした文字列に変換
        if let Ok(num) = numeric_part.parse::<u32>() {
            Some(format!("{:010}", num)) // 10桁の0パディング
        } else {
            None
        }
    }

    /// 画像ファイルを処理する（WebPの場合はJPEG変換）
    ///
    /// # Arguments
    /// * `file_path` - 画像ファイルのパス
    ///
    /// # Returns
    /// * `Result<DynamicImage>` - 処理済みの画像
    fn process_image(&self, file_path: &Path) -> Result<DynamicImage> {
        let img = image::open(file_path)?;
        
        // WebPの場合はJPEG形式に変換処理をログ出力
        if let Some(extension) = file_path.extension() {
            let ext_str = extension.to_string_lossy().to_lowercase();
            if ext_str == "webp" {
                debug!("Processing WebP image (will be converted to JPEG): {:?}", file_path);
            }
        }

        Ok(img)
    }

    /// 画像をPDFページとして追加する
    ///
    /// # Arguments
    /// * `pdf` - PDFライター
    /// * `page_id` - ページID
    /// * `img` - 追加する画像
    /// * `next_id` - 次に使用するID（更新される）
    ///
    /// # Returns
    /// * `Result<()>` - 処理の成功/失敗
    fn add_image_as_page(
        &self, 
        pdf: &mut Pdf, 
        page_id: Ref, 
        img: &DynamicImage, 
        next_id: &mut i32
    ) -> Result<()> {
        // 画像のサイズを取得
        let (img_width, img_height) = img.dimensions();
        
        // 72 DPIでのページサイズ計算（ポイント単位）
        let dpi = 72.0_f32;
        let page_width = img_width as f32 / dpi * 72.0;
        let page_height = img_height as f32 / dpi * 72.0;
        
        // 画像をJPEGバイトデータに変換
        let image_bytes = self.image_to_jpeg_bytes(img)?;
        
        // 画像XObjectのID
        let image_id = Ref::new(*next_id);
        *next_id += 1;
        
        // コンテンツストリームのID
        let content_id = Ref::new(*next_id);
        *next_id += 1;
        
        // 画像XObjectを作成
        let mut image_obj = pdf.image_xobject(image_id, &image_bytes);
        image_obj.filter(Filter::DctDecode);
        image_obj.width(img_width as i32);
        image_obj.height(img_height as i32);
        image_obj.color_space().device_rgb();
        image_obj.bits_per_component(8);
        image_obj.finish();
        
        // コンテンツストリームを作成
        let content = format!(
            "q\n{} 0 0 {} 0 0 cm\n/Im1 Do\nQ",
            page_width, page_height
        );
        pdf.stream(content_id, content.as_bytes());
        
        // ページを作成
        let mut page = pdf.page(page_id);
        page.media_box(Rect::new(0.0, 0.0, page_width, page_height));
        page.contents(content_id);
        let mut resources = page.resources();
        let mut xobjects = resources.x_objects();
        xobjects.pair(Name(b"Im1"), image_id);
        xobjects.finish();
        resources.finish();
        page.finish();
        
        debug!("Added image as page: {}x{} ({}x{} points)", img_width, img_height, page_width, page_height);
        
        Ok(())
    }

    /// 画像をJPEGバイト配列に変換する
    ///
    /// # Arguments
    /// * `img` - 変換する画像
    ///
    /// # Returns
    /// * `Result<Vec<u8>>` - 画像のJPEGバイト配列
    fn image_to_jpeg_bytes(&self, img: &DynamicImage) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        let mut cursor = Cursor::new(&mut buffer);
        
        // JPEG形式でエンコード（品質80%）
        img.write_to(&mut cursor, ImageOutputFormat::Jpeg(80))?;
        
        Ok(buffer)
    }
}

impl Default for ImageToPdfConverter {
    fn default() -> Self {
        Self::new()
    }
}