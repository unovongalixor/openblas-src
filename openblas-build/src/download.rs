use anyhow::Result;
use std::path::{Path, PathBuf};
use ureq::{
    config::Config,
    tls::{TlsConfig, TlsProvider},
};

const OPENBLAS_VERSION: &str = "0.3.30";

pub fn openblas_source_url() -> String {
    format!(
        "https://github.com/OpenMathLib/OpenBLAS/releases/download/v{}/OpenBLAS-{}.tar.gz",
        OPENBLAS_VERSION, OPENBLAS_VERSION
    )
}

pub fn download(out_dir: &Path) -> Result<PathBuf> {
    let dest = out_dir.join(format!("OpenBLAS-{}", OPENBLAS_VERSION));
    if !dest.exists() {
        let buf = get_agent()
            .get(&openblas_source_url())
            .call()?
            .into_body()
            .into_reader();
        let gz_stream = flate2::read::GzDecoder::new(buf);
        let mut ar = tar::Archive::new(gz_stream);
        ar.unpack(out_dir)?;
        assert!(dest.exists());

        // Read the file content
        let content = std::fs::read_to_string(dest.join("utest").join("ctest.h"))?;

        // Perform the string replacement
        let modified_content = content.replace("#ifndef __CTEST_NO_TIME", "#if !defined(__CTEST_NO_TIME) && !defined(CTEST_NO_INTTYPES)");

        // Write the modified content back to the file
        std::fs::write(dest.join("utest").join("ctest.h"), modified_content.as_bytes())?;
    }
    Ok(dest)
}

fn get_agent() -> ureq::Agent {
    Config::builder()
        .tls_config(
            TlsConfig::builder()
                .provider(TlsProvider::NativeTls)
                .build(),
        )
        .build()
        .new_agent()
}
