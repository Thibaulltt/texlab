#[derive(Debug, Clone, Default)]
pub struct LatexmkrcData {
    pub cwd: Option<String>,
    pub aux_dir: Option<String>,
    pub out_dir: Option<String>,
}
