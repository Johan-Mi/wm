#[macro_use]
extern crate penrose;
mod defs;
mod layouts;
use defs::*;
use layouts::get_layouts;

use penrose::{
    core::helpers::index_selectors, logging_error_handler,
    xcb::new_xcb_backed_window_manager, Backward, Config, Forward, Less, More,
    Selector,
};

fn main() -> penrose::Result<()> {
    let key_bindings = gen_keybindings! {
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-h" => run_internal!(update_main_ratio, Less);
        "M-l" => run_internal!(update_main_ratio, More);
        "M-d" => run_internal!(update_max_main, Less);
        "M-S-d" => run_internal!(update_max_main, More);
        "M-f" => run_internal!(toggle_client_fullscreen, &Selector::Focused);
        "M-S-c" => run_internal!(kill_client);
        "M-S-q" => run_internal!(exit);
        "M-p" => run_external!("dmenu_run");
        "A-C-l" => run_external!("slock");
        "M-Return" => run_external!(TERMINAL);
        "M-C-y" => run_external!("st -e python");
        "M-i" => run_external!(BROWSER);
        "M-y" => run_external!("/usr/lib/brave-bin/brave \
                               --new-window youtube.com");
        "M-A-q" => run_external!("/home/johanmi/.local/bin/change-volume 1%-");
        "M-A-w" => run_external!("/home/johanmi/.local/bin/change-volume 1%-");
        "M-A-a" => run_external!("brightnessctl set 1%-");
        "M-A-s" => run_external!("brightnessctl set 1%+");
        "M-x" => run_external!("xdotool mousemove 675 757");
        "M-A-h" => run_external!("xdotool mousemove_relative -- -10 0");
        "M-A-l" => run_external!("xdotool mousemove_relative 10 0");
        "M-A-k" => run_external!("xdotool mousemove_relative -- 0 -10");
        "M-A-j" => run_external!("xdotool mousemove_relative 0 10");
        "M-A-u" => run_external!("xdotool click --clearmodifiers 1");
        "M-A-i" => run_external!("xdotool click --clearmodifiers 3");
        "M-A-o" => run_external!("xdotool click --clearmodifiers 2");
        "M-A-y" => run_external!("xdotool click --clearmodifiers 5");
        "M-A-p" => run_external!("xdotool click --clearmodifiers 4");
        "M-C-p" => run_external!("scrot %Y-%m-%d-%H-%M-%S_$wx$h.png \
                                 -e mv $f ~/Pictures/");
        "M-C-e" => run_external!("/home/johanmi/.local/bin/dmenuunicode");
        "M-C-r" => run_external!("/home/johanmi/.local/bin/subreddits");
        "C-A-Delete" => run_external!("mpv --fs /usr/local/etc/rickroll.mp4");

        refmap [1..10] in {
            "M-{}" => focus_workspace[index_selectors(9)];
            "M-S-{}" => client_to_workspace[index_selectors(9)];
        };
    };

    let mut config_builder = Config::default().builder();
    let config = config_builder
        .layouts(get_layouts())
        .border_px(BORDER_SIZE)
        .focused_border(BORDER_COLOR_FOCUSED)
        .unfocused_border(BORDER_COLOR_UNFOCUSED)
        .gap_px(GAP_SIZE)
        .main_ratio_step(0.0125)
        .build()
        .unwrap();

    let mut wm =
        new_xcb_backed_window_manager(config, vec![], logging_error_handler())?;

    wm.grab_keys_and_run(key_bindings, map! {})
}
