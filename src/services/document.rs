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
/// * `order` - The order to create the invoice for
/// # Returns
///
/// A byte vector containing the invoice
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
