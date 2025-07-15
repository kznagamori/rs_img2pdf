use clap::Parser;
use log::{error, info};
use std::path::PathBuf;

mod converter;
mod error;
mod logger;

use converter::ImageToPdfConverter;
use error::Result;

/// 複数の画像ファイルを1つのPDFファイルに変換するツール
#[derive(Parser)]
#[command(name = "rs_img2pdf")]
#[command(version = "0.1.0")]
#[command(about = "Convert multiple image files to a single PDF")]
#[command(long_about = None)]
struct Args {
    /// 入力ディレクトリのパス（指定しない場合はカレントディレクトリ）
    #[arg(short, long)]
    input: Option<PathBuf>,

    /// 出力PDFファイルのパス（指定しない場合はディレクトリ名.pdf）
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// ログレベル
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// 詳細ログを有効にする
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // ログの初期化
    let log_level = if args.verbose {
        "debug"
    } else {
        &args.log_level
    };
    logger::init_logger(log_level)?;

    info!("rs_img2pdf started");

    // 入力ディレクトリの決定
    let input_dir = args.input.unwrap_or_else(|| {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    });

    // 出力ファイル名の決定
    let output_file = args.output.unwrap_or_else(|| {
        let dir_name = input_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("output");
        PathBuf::from(format!("{}.pdf", dir_name))
    });

    info!("Input directory: {:?}", input_dir);
    info!("Output file: {:?}", output_file);

    // 変換処理の実行
    let converter = ImageToPdfConverter::new();
    match converter.convert(&input_dir, &output_file) {
        Ok(_) => {
            info!("PDF conversion completed successfully: {:?}", output_file);
        }
        Err(e) => {
            error!("PDF conversion failed: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}