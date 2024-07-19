use regex::Regex;
use crate::comment_remover::CommentRemover;

pub struct PythonCommentRemover;

impl CommentRemover for PythonCommentRemover {
    fn remove_comments(&self, code: &str) -> (String, usize) {
        let single_line = Regex::new(r"#.*").unwrap();
        let multi_line = Regex::new(r"'''[\s\S]*?'''").unwrap();

        let single_line_matches: Vec<_> = single_line.find_iter(code).collect();
        let multi_line_matches: Vec<_> = multi_line.find_iter(code).collect();

        let without_single_line = single_line.replace_all(code, "");
        let cleaned_code = multi_line.replace_all(&without_single_line, "").into_owned();

        let lines_removed = single_line_matches.len() +
            multi_line_matches.iter().map(|m| m.as_str().lines().count()).sum::<usize>();

        (cleaned_code, lines_removed)
    }
}