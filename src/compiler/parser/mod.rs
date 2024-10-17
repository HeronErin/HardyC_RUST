pub mod tokenizer;
pub mod translation;

pub mod string_patch_resolver;

pub fn gen_line_map(input: &str) -> Vec<usize> {
    input
        .char_indices()
        .filter(|x| x.1 == '\n')
        .map(|x| x.0)
        .collect()
}
