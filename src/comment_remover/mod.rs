mod javascript_comment_remover;
mod python_comment_remover;
mod rust_comment_remover;

use crate::comment_remover::javascript_comment_remover::JavaScriptCommentRemover;
use crate::comment_remover::python_comment_remover::PythonCommentRemover;
use crate::comment_remover::rust_comment_remover::RustCommentRemover;

pub trait CommentRemover {
    fn remove_comments(&self, code: &str) -> (String, usize);
}

pub fn get_comment_remover(file_extension: &str) -> Box<dyn CommentRemover> {
    match file_extension {
        "js" | "ts" => Box::new(JavaScriptCommentRemover),
        "py" => Box::new(PythonCommentRemover),
        "rs" => Box::new(RustCommentRemover),
        _ => Box::new(JavaScriptCommentRemover), // Default to JavaScript remover
    }
}
