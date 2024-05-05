use emacs::{defun, Env, Result, Value, Vector};
use git2::Repository;
// https://egh0bww1.com/posts/2022-09-09-22-emacs-symbol-implement/#orgnh-4.4
// symbol既然都是对象了，那肯定不是一个简单值。在 elisp 中，symbol 由四部分组成：
// 1.print value ，symbol 的名字
// 2.value ，symbol 中保存的值，这一部分也叫 value cell
// 3.function ，symbol 中保存的函数，这一部分也叫 function cell
// 4.property list ，保存属性的表，也叫 plist

// 定义一个 C int 类型的名为 plugin_is_GPL_compatible 的全局变量，否则 Emacs 会拒绝加
// 载模块 :)
emacs::plugin_is_GPL_compatible!();

// 定义模块入口，大部份情况我们不需要它做任何事。selfn 是自定义模块名前缀
#[emacs::module(name = "selfn")]
fn init(_: &Env) -> Result<()> {
    Ok(())
}
/// 一个愚蠢但是简单的例子，这个注释会被导入 Emacs 的帮助系统中，
/// 通过 M-x describe-function 查阅。
#[defun]
fn add(lhs: i64, rhs: i64) -> emacs::Result<i64> {
    Ok(lhs + rhs)
}
/// 函数doc: 函数作用 project-say-hello
#[defun]
fn say_hello(env: &Env, name: String) -> Result<Value<'_>> {
    env.message(&format!("Hello, {}!", name))
}

/// 是的，这函数将一个字符串 intern 为 Elisp symbol。但是你为什么不直接在 Elisp 里
/// 用 intern 呢
#[defun]
fn silly_intern<'e>(env: &'e Env, s: String) -> emacs::Result<Value<'e>> {
    env.intern(s.as_ref())
}

/// 函数doc: 函数作用 selfn-say-hello
#[defun]
fn say_none(env: &Env, name: String) -> Result<Value<'_>> {
    // (list "str" 2)
    env.call("list", ("str", 2))?;

    let list = env.intern("list")?;
    // (symbol-function 'list)
    let subr = env.call("symbol-function", [list])?;
    // (funcall 'list "str" 2)
    env.call(list, ("str", 2))?;
    // (funcall (symbol-function 'list) "str" 2)
    env.call(subr, ("str", 2))?;
    subr.call(("str", 2))?; // 简写

    // (add-hook 'text-mode-hook 'variable-pitch-mode)
    env.call(
        "add-hook",
        [
            env.intern("text-mode-hook")?,
            env.intern("variable-pitch-mode")?,
        ],
    )?;

    env.message(&format!("Hello, {}!", name))
}

// vector 转list
#[defun]
fn listify_vec<'a>(env: &'a Env, vector: Vector<'a>) -> Result<Value<'a>> {
    let mut args = vec![];
    for i in 0..vector.len() {
        args.push(vector.get(i)?)
    }
    env.call("list", &args)
}

/// Parse the given rev using libgit2.
#[defun]
fn rev_parse<'a>(env: &'a Env, path: String, spec: String) -> Result<String> {
    let repo = Repository::discover(&path)?;
    let obj = repo.revparse_single(&spec)?;
    env.message(&format!("Hello, {}!", path))?;
    Ok(obj.id().to_string())
}
