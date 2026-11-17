use std::str::FromStr;

use imitare::Extension;

#[test]
fn test_extension_from_str() {
    assert_eq!(Extension::from_str("zip").unwrap(), Extension::Zip);
    assert_eq!(Extension::from_str("pdf").unwrap(), Extension::Pdf);
    assert_eq!(Extension::from_str("doc").unwrap(), Extension::Doc);
    assert_eq!(Extension::from_str("txt").unwrap(), Extension::Txt);
    assert!(Extension::from_str("unknown").is_err());
}

#[test]
fn test_extension_headers() {
    // ZIP header test - should start with PK signature
    let zip_header = Extension::Zip.header();
    assert_eq!(zip_header[0..4], [0x50, 0x4b, 0x03, 0x04]);

    // PDF header test - should start with %PDF-1.4
    let pdf_header = Extension::Pdf.header();
    assert_eq!(
        pdf_header[0..8],
        [0x25, 0x50, 0x44, 0x46, 0x2d, 0x31, 0x2e, 0x34]
    );

    // DOC header test - should match compound document signature
    let doc_header = Extension::Doc.header();
    assert_eq!(
        doc_header,
        vec![0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1]
    );

    // TXT header test - should be empty
    let txt_header = Extension::Txt.header();
    assert!(txt_header.is_empty());
}
