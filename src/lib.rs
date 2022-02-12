//////pub mod bash_like_utils;

use std::path::{Path, PathBuf};
use std::ffi::OsStr;

/// Internal empty struct for implementation only
#[derive(Debug, Clone)]
pub struct PathString;

/// Some functions to operate with pathnames (as in bash)
impl PathString {
    /// like bash::readlink
    pub fn readlink(path: &str) -> String {
        let path = Path::new(path).canonicalize().unwrap_or(PathBuf::new());
        path.to_str().unwrap_or("").to_string()
    }
    /// like bash::dirname
    pub fn dirname(path: &str) -> String {
        let path = Path::new(path).parent().unwrap_or(Path::new(""));
        path.to_str().unwrap_or("").to_string()
    }
    /// like bash::basename
    pub fn basename(path: &str) -> String {
        let path = Path::new(path).file_name().unwrap_or(OsStr::new(""));
        path.to_str().unwrap_or("").to_string()
    }
    /// no alias in bash: returns extension of the path
    pub fn extension(path: &str) -> String {
        let bn = PathString::basename(path);
        let bn : Vec<_> = bn.split(".").collect();
        // dbg!(bn.len());
        match bn.len() {   0..=1 => String::new(),  _ => bn.last().unwrap().to_string()   }
    }
    /// no alias in bash: returns basename without extension
    pub fn basenoext(path: &str) -> String {
        let bn = PathString::basename(path);
        let mut bn : Vec<_> = bn.split(".").collect();
        // dbg!(bn.len());
        match bn.len() {   0 => String::new(), 1 => { bn[0].to_string() },  _ => { bn.pop(); bn.last().unwrap().to_string() } }
    }
    /// no alias in bash: returns basename without extension (calls basenoext)
    pub fn file_stem(path: &str) -> String {
        PathString::basenoext(path)
    }
}
