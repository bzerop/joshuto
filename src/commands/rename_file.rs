use std::path;

use crate::context::AppContext;
use crate::error::JoshutoResult;
use crate::ui::TuiBackend;
use crate::util::load_child::LoadChild;

use super::command_line;

pub fn _rename_file(
    context: &mut AppContext,
    src: &path::Path,
    dest: &path::Path,
) -> std::io::Result<()> {
    let new_path = dest;
    if new_path.exists() {
        let err = std::io::Error::new(std::io::ErrorKind::AlreadyExists, "Filename already exists");
        return Err(err);
    }
    std::fs::rename(&src, &dest)?;
    let options = context.config_ref().display_options_ref().clone();
    if let Some(curr_list) = context.tab_context_mut().curr_tab_mut().curr_list_mut() {
        curr_list.reload_contents(&options)?;
    }
    Ok(())
}

pub fn rename_file(context: &mut AppContext, dest: &path::Path) -> JoshutoResult<()> {
    let path: Option<path::PathBuf> = context
        .tab_context_ref()
        .curr_tab_ref()
        .curr_list_ref()
        .and_then(|s| s.curr_entry_ref())
        .map(|s| s.file_path().to_path_buf());

    if let Some(path) = path {
        _rename_file(context, path.as_path(), dest)?;
    }
    LoadChild::load_child(context)?;
    Ok(())
}

pub fn _rename_file_append(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    file_name: &str,
) -> JoshutoResult<()> {
    let (prefix, suffix): (String, String) = match file_name.rfind('.') {
        Some(ext) => (
            format!("rename {}", &file_name[0..ext]),
            file_name[ext..].to_string(),
        ),
        None => (format!("rename {}", file_name), "".to_string()),
    };
    command_line::readline(context, backend, &prefix, &suffix)
}

pub fn rename_file_append(context: &mut AppContext, backend: &mut TuiBackend) -> JoshutoResult<()> {
    let mut file_name: Option<String> = None;

    if let Some(curr_list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        file_name = curr_list
            .curr_entry_ref()
            .map(|s| s.file_name().to_string());
    }

    if let Some(file_name) = file_name {
        _rename_file_append(context, backend, file_name.as_str())?;
    }
    Ok(())
}

pub fn _rename_file_prepend(
    context: &mut AppContext,
    backend: &mut TuiBackend,
    file_name: String,
) -> JoshutoResult<()> {
    let prefix = String::from("rename ");
    let suffix = file_name;
    command_line::readline(context, backend, &prefix, &suffix)
}

pub fn rename_file_prepend(
    context: &mut AppContext,
    backend: &mut TuiBackend,
) -> JoshutoResult<()> {
    let mut file_name: Option<String> = None;

    if let Some(curr_list) = context.tab_context_ref().curr_tab_ref().curr_list_ref() {
        file_name = curr_list
            .curr_entry_ref()
            .map(|s| s.file_name().to_string());
    }

    if let Some(file_name) = file_name {
        _rename_file_prepend(context, backend, file_name)?;
    }
    Ok(())
}
