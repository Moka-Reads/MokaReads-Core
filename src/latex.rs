use pandoc::{
    InputFormat, InputKind, MarkdownExtension, OutputFormat, OutputKind, Pandoc, PandocOption,
    PandocOutput,
};
use std::error::Error;
use std::path::PathBuf;

/// Build types to turn a markdown file to:
/// - A LaTeX file
/// - A PDF file with `listings` code blocks
/// - A PDF file with the `listings-rust` styling
#[derive(Debug, Clone, Copy)]
pub enum BuildType {
    TeX,
    PDF,
    PdfRustStyling,
}

/// Gets the listings-rust style from `denki/listings-rust`
pub async fn get_rust_sty() -> Result<(), Box<dyn Error>> {
    let url = "https://raw.githubusercontent.com/denki/listings-rust/master/listings-rust.sty";
    let client = reqwest::Client::new().get(url).send().await?;
    let data = client.text().await?;
    tokio::fs::write("listings-rust.sty", data).await?;
    Ok(())
}

/// Converts a Markdown file to a standalone latex string
pub fn to_latex_string(markdown: &str) -> String {
    let mut pd = Pandoc::new();
    // make the latex standalone
    let options = [PandocOption::Standalone, PandocOption::Listings];
    pd.add_options(&options);
    // set the input format to markdown
    pd.set_input_format(InputFormat::Markdown, vec![]);
    // set the output format to latex
    pd.set_output_format(OutputFormat::Latex, vec![]);
    // set the input string
    pd.set_input(InputKind::Pipe(markdown.to_string()));
    // execute the conversion
    pd.set_output(OutputKind::Pipe);
    let pd_out = pd.execute().unwrap();
    match pd_out {
        PandocOutput::ToBuffer(s) => s,
        _ => "".to_string(),
    }
}

pub async fn build(
    markdown: &str,
    file_name: &str,
    out_path: PathBuf,
    build_ty: BuildType,
) -> Result<(), Box<dyn Error>> {
    let mut pd = Pandoc::new();
    // make the latex standalone
    let options = [
        PandocOption::Standalone,
        PandocOption::Listings,
        PandocOption::HighlightStyle("tango".to_string()),
    ];
    pd.add_options(&options);
    // set the input format to markdown
    pd.set_input_format(
        InputFormat::Markdown,
        vec![MarkdownExtension::FencedCodeBlocks],
    );
    // set the input string
    pd.set_input(InputKind::Pipe(markdown.to_string()));

    match build_ty {
        BuildType::TeX => {
            let path = out_path.join(&format!("{file_name}.tex"));
            pd.set_output_format(OutputFormat::Latex, vec![]);
            pd.set_output(OutputKind::File(path));
        }
        BuildType::PDF => {
            let path = out_path.join(&format!("{file_name}.pdf"));
            pd.set_output_format(OutputFormat::Pdf, vec![]);
            pd.set_output(OutputKind::File(path));
        }
        BuildType::PdfRustStyling => {
            get_rust_sty().await?;
            pd.add_option(PandocOption::IncludeInHeader(PathBuf::from(
                "listings-rust.sty",
            )));
            let path = out_path.join(&format!("{file_name}.pdf"));
            pd.set_output_format(OutputFormat::Pdf, vec![]);
            pd.set_output(OutputKind::File(path));
        }
    }

    let _ = pd.execute()?;
    Ok(())
}
