//! Library bash_like_utils;
//!
//! Contains struct PathString with some functions that behave like bash utilities

use std::path::{Path, PathBuf};
use std::ffi::OsStr;

/// Internal empty struct for implementation only
#[derive(Debug, Clone)]
pub struct PathString;

/// Some functions to operate with pathnames (as in bash)
impl PathString {
    /// like bash::readlink
    /// readlink acts as readlink in bash
    /// 
    /// # Example
    /// ```
    /// let q = bash_like_utils::PathString::readlink("./qweqwe");
    /// assert_ne!(q,"./qweqwe");
    /// ```
    pub fn readlink(path: &str) -> String {
///////! inner doc string of `readlink`
        let path = Path::new(path).canonicalize().unwrap_or(PathBuf::new());
        path.to_str().unwrap_or("").to_string()
    }
    /// like bash::dirname
    ///
    /// dirname returns dir-part of a full pathname
    /// 
    /// # Example
    /// ```
    /// let q = bash_like_utils::PathString::dirname("./qweqwe");
    /// assert_eq!(q,".");
    /// let q = bash_like_utils::PathString::dirname("/qwe/asd/zxczxc");
    /// assert_eq!(q,"/qwe/asd");
    /// let q = bash_like_utils::PathString::dirname("qwe/asd/zxczxc");
    /// assert_eq!(q,"qwe/asd");
    /// let q = bash_like_utils::PathString::dirname("/qwe/asd/zxczxc/");
    /// assert_eq!(q,"/qwe/asd");
    /// ```
    pub fn dirname(path: &str) -> String {
        let path = Path::new(path).parent().unwrap_or(Path::new(""));
        path.to_str().unwrap_or("").to_string()
    }
    /// like bash::basename
    ///
    /// basename returns filename-part of a full pathname
    /// 
    /// # Example
    /// ```
    /// let q = bash_like_utils::PathString::basename("./qweqwe");
    /// assert_eq!(q,"qweqwe");
    /// let q = bash_like_utils::PathString::basename("/qwe/asd/zxczxc");
    /// assert_eq!(q,"zxczxc");
    /// let q = bash_like_utils::PathString::basename("qwe/asd/zxczxc");
    /// assert_eq!(q,"zxczxc");
    /// let q = bash_like_utils::PathString::basename("/qwe/asd/zxczxc/");
    /// assert_eq!(q,"zxczxc");
    /// ```
    pub fn basename(path: &str) -> String {
        let path = Path::new(path).file_name().unwrap_or(OsStr::new(""));
        path.to_str().unwrap_or("").to_string()
    }
    /// no alias in bash: returns extension of the path
    /// 
    /// # Example
    /// ```
    /// let q = bash_like_utils::PathString::extension("./qweqwe");
    /// assert_eq!(q,"");
    /// let q = bash_like_utils::PathString::extension("/qwe/asd/zxczxc");
    /// assert_eq!(q,"");
    /// let q = bash_like_utils::PathString::extension("/qwe/asd/zxczxc.wer");
    /// assert_eq!(q,"wer");
    /// let q = bash_like_utils::PathString::extension("qwe/asd/zxczxc");
    /// assert_eq!(q,"");
    /// let q = bash_like_utils::PathString::extension("qwe/asd/zxczxc.wer");
    /// assert_eq!(q,"wer");
    /// let q = bash_like_utils::PathString::extension("/qwe/asd/zxczxc/");
    /// assert_eq!(q,"");
    /// let q = bash_like_utils::PathString::extension("/qwe/asd/zxczxc.wer/");
    /// assert_eq!(q,"wer");
    /// ```
    pub fn extension(path: &str) -> String {
        let bn = PathString::basename(path);
        let bn : Vec<_> = bn.split(".").collect();
        // dbg!(bn.len());
        match bn.len() {   0..=1 => String::new(),  _ => bn.last().unwrap().to_string()   }
    }
    /// no alias in bash: returns basename without extension
    /// 
    /// # Example
    /// ```
    /// let q = bash_like_utils::PathString::basenoext("./qweqwe");
    /// assert_eq!(q,"qweqwe");
    /// let q = bash_like_utils::PathString::basenoext("/qwe/asd/zxczxc");
    /// assert_eq!(q,"zxczxc");
    /// let q = bash_like_utils::PathString::basenoext("/qwe/asd/zxczxc.wer");
    /// assert_eq!(q,"zxczxc");
    /// let q = bash_like_utils::PathString::basenoext("qwe/asd/zxczxc");
    /// assert_eq!(q,"zxczxc");
    /// let q = bash_like_utils::PathString::basenoext("qwe/asd/zxczxc.wer");
    /// assert_eq!(q,"zxczxc");
    /// let q = bash_like_utils::PathString::basenoext("/qwe/asd/zxczxc/");
    /// assert_eq!(q,"zxczxc");
    /// let q = bash_like_utils::PathString::basenoext("/qwe/asd/zxczxc.wer/");
    /// assert_eq!(q,"zxczxc");
    /// ```
    pub fn basenoext(path: &str) -> String {
        let bn = PathString::basename(path);
        let mut bn : Vec<_> = bn.split(".").collect();
        // dbg!(bn.len());
        match bn.len() {   0 => String::new(), 1 => { bn[0].to_string() },  _ => { bn.pop(); bn.last().unwrap().to_string() } }
    }
    /// no alias in bash: returns basename without extension (calls basenoext)
    ///
    /// Alias to `basenoext` (calls it from inside)
    pub fn file_stem(path: &str) -> String {
        PathString::basenoext(path)
    }
}
