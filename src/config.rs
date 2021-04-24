use rustyline::config::*;

struct YuiConfig {
    hist_ign_space: bool,
    hist_ign_dups: bool,
    hist_max_size: usize,
    completion_type: CompletionType,
    completion_limit: usize,
    keyseq_timeout: i32,
    edit_mode: EditMode,
    auto_add_history: bool,
    bell_style: BellStyle,
    color_mode: ColorMode,
    output_stream: OutputStreamType,
    tab_stop: usize,
    check_cur_pos: bool,
    indent_size: usize,
    bracketed_paste: bool,
}
