use printpdf::{PdfDocument, PdfSaveOptions, XmlRenderOptions};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Pdf error")]
    PdfError,
}

/// Returns the invoice as a byte vector
///
/// # Arguments
///
/// * `content` - The content to create the pdf for
/// * `document_name` - The name of the document
/// # Returns
///
/// A byte vector containing the pdf
///
/// # Errors
///
/// Returns an `InvoiceError` if the invoice cannot be created
pub fn create_pdf(content: &str, document_name: &str) -> Result<Vec<u8>, Error> {
    let option = XmlRenderOptions { ..Default::default() };
    let document = PdfDocument::new(document_name)
        .with_html(content, option)
        .map_err(|_| Error::PdfError)?
        .save(&PdfSaveOptions::default());

    Ok(document)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_pdf_success() {
        let html_content = r#"
           <!DOCTYPE html>
            <html>
            <body>
                <div>
                    <p>Thank You for Your Order!</p>
                </div>
            </body>
            </html>
        "#;

        let result = create_pdf(html_content, "test_document");
        assert!(result.is_ok());

        let pdf_bytes = result.unwrap();
        assert!(!pdf_bytes.is_empty());
    }

    #[tokio::test]
    async fn test_create_pdf_invalid_html() {
        let invalid_html = "<invalid>";
        let result = create_pdf(invalid_html, "test_document");
        assert!(matches!(result, Err(Error::PdfError)));
    }
}
