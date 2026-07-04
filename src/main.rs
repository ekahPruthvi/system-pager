use gtk4::pango::BidiType::B;
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Orientation, Button, Label, CssProvider};
use gtk4::gdk::Display;
use gtk4_layer_shell::{LayerShell, Layer, Edge};
use std::process::exit;
use std::{time::Duration, thread};
use chrono::{Datelike, Local};

fn main() {
    let app = Application::builder()
        .application_id("ekah.scu.pager")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::new(app);
    window.init_layer_shell();
    window.set_layer(Layer::Overlay);
    window.fullscreen();
    window.set_decorated(false);
    window.set_namespace(Some("systemPager"));
    window.set_keyboard_mode(gtk4_layer_shell::KeyboardMode::Exclusive);

    for (edge, anchor) in [
        (Edge::Left, true),
        (Edge::Right, true),
        (Edge::Top, true),
        (Edge::Bottom, true),
    ] {
        window.set_anchor(edge, anchor);
    }

    let css = CssProvider::new();
    css.load_from_data(
        "
        window {
            background-color: rgba(0, 0, 0, 0);
        }

        .mainwin {
            background-color: rgba(24, 24, 24, 0.1);
        }

        .error {
            background-color: rgb(0, 26, 255);
            padding: 20px;
        }
        
        .pgone {
            --color1: rgb(108, 231, 221);
            --color2: rgb(250, 255, 178);
            background-color: var(--color1);
            background-image: linear-gradient(45deg, var(--color2) 25%, transparent 25%, transparent 75%, var(--color2) 75%, var(--color2)), 
                                linear-gradient(45deg, var(--color2) 25%, var(--color1) 25%, var(--color1) 75%, var(--color2) 75%, var(--color2));
            background-size: 60px 60px;
            background-position: 0 0, 30px 30px;  
        }

        .jk {
            font-size: 55px;
            font-weight: 900;
            text-shadow: 
                0px  10px 0 #FFFF00,  0px -10px 0 #FFFF00, 10px  0px 0 #FFFF00, -10px  0px 0 #FFFF00,
                7px   7px 0 #FFFF00, -7px   7px 0 #FFFF00,  7px -7px 0 #FFFF00,  -7px  -7px 0 #FFFF00,
                5px   9px 0 #FFFF00, -5px   9px 0 #FFFF00,  5px -9px 0 #FFFF00,  -5px  -9px 0 #FFFF00,
                9px   5px 0 #FFFF00, -9px   5px 0 #FFFF00,  9px -5px 0 #FFFF00,  -9px  -5px 0 #FFFF00,
                2px  10px 0 #FFFF00, -2px  10px 0 #FFFF00,  2px -10px 0 #FFFF00,  -2px -10px 0 #FFFF00,
                10px   2px 0 #FFFF00, -10px  2px 0 #FFFF00, 10px -2px 0 #FFFF00,  -10px -2px 0 #FFFF00,
                0px   5px 0 #FFFF00,  0px  -5px 0 #FFFF00,  5px  0px 0 #FFFF00,   -5px  0px 0 #FFFF00,
                4px   4px 0 #FFFF00, -4px   4px 0 #FFFF00,  4px -4px 0 #FFFF00,   -4px -4px 0 #FFFF00;
            color: blue;
            transform: rotate(1.5deg);
        }

        .bday {
            font-family: 'Courier New', 'Courier', 'Monospace', monospace;
            font-size: 11px;            
            line-height: 1.0;         
            letter-spacing: 0px; 
            white-space: pre;
            text-shadow: 
                0px  10px 0 rgb(133, 131, 0),  0px -10px 0 rgb(133, 131, 0), 10px  0px 0 rgb(133, 131, 0), -10px  0px 0 rgb(133, 131, 0),
                7px   7px 0 rgb(133, 131, 0), -7px   7px 0 rgb(133, 131, 0),  7px -7px 0 rgb(133, 131, 0),  -7px  -7px 0 rgb(133, 131, 0),
                5px   9px 0 rgb(133, 131, 0), -5px   9px 0 rgb(133, 131, 0),  5px -9px 0 rgb(133, 131, 0),  -5px  -9px 0 rgb(133, 131, 0),
                9px   5px 0 rgb(133, 131, 0), -9px   5px 0 rgb(133, 131, 0),  9px -5px 0 rgb(133, 131, 0),  -9px  -5px 0 rgb(133, 131, 0),
                2px  10px 0 rgb(133, 131, 0), -2px  10px 0 rgb(133, 131, 0),  2px -10px 0 rgb(133, 131, 0),  -2px -10px 0 rgb(133, 131, 0),
                10px   2px 0 rgb(133, 131, 0), -10px  2px 0 rgb(133, 131, 0), 10px -2px 0 rgb(133, 131, 0),  -10px -2px 0 rgb(133, 131, 0),
                0px   5px 0 rgb(133, 131, 0),  0px  -5px 0 rgb(133, 131, 0),  5px  0px 0 rgb(133, 131, 0),   -5px  0px 0 rgb(133, 131, 0),
                4px   4px 0 rgb(133, 131, 0), -4px   4px 0 rgb(133, 131, 0),  4px -4px 0 rgb(133, 131, 0),   -4px -4px 0 rgb(133, 131, 0);
            color: #ffff00;
            transform: rotate(-0.5deg);
        }

        .bdaybox {
            border-radius: 10px;
            border: 2px solid rgba(0, 0, 0, 0.92);
            background-color: #0f172a;
  background-image: linear-gradient(
      45deg,
      rgba(59, 130, 246, 0.08) 25%,
      transparent 25%,
      transparent 75%,
      rgba(59, 130, 246, 0.08) 75%
    ),
    linear-gradient(
      -45deg,
      rgba(59, 130, 246, 0.08) 25%,
      transparent 25%,
      transparent 75%,
      rgba(59, 130, 246, 0.08) 75%
    ),
    linear-gradient(
      45deg,
      transparent 40%,
      rgba(99, 102, 241, 0.1) 40%,
      rgba(99, 102, 241, 0.1) 60%,
      transparent 60%
    ),
    linear-gradient(
      -45deg,
      transparent 40%,
      rgba(99, 102, 241, 0.1) 40%,
      rgba(99, 102, 241, 0.1) 60%,
      transparent 60%
    ),
    radial-gradient(circle at 50% 50%, #1e293b 0%, #0f172a 100%);
  background-size:
    60px 60px,
    60px 60px,
    120px 120px,
    120px 120px,
    100% 100%;
  background-position:
    0 0,
    30px 30px,
    0 0,
    60px 60px,
    0 0;
  position: relative;
            padding: 30px 50px 30px 0px;
            box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
            transform-origin: bottom;
            animation: scaleUpFromBottom 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
        }

        @keyframes scaleUpFromBottom {
            from {
                transform: scaleY(0);
                opacity: 0;
            }
            to {
                transform: scaleY(1);
                opacity: 1;
            }
        }

        .exp {
            border-radius: 50px;
            border: 1px solid rgba(219, 219, 219, 0.16);
            background-color: rgba(36, 36, 36, 0.88);
            padding: 10px;
            box-shadow: rgba(0, 0, 0, 0.35) 0px 5px 15px;
            font-weight: 300;
            font-size: 20px;
            animation: slowFloat 4s ease-in-out infinite;
            color: rgba(255, 255, 255, 0.6);
        }

        @keyframes slowFloat {
            0% {
                transform: translateY(0px);
            }
            50% {
                transform: translateY(-15px); /* Drift upwards by 15 pixels */
            }
            100% {
                transform: translateY(0px);  /* Return smoothly to starting position */
            }
        }

    ",
    );

    gtk4::style_context_add_provider_for_display(
        &Display::default().unwrap(),
        &css,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let today = Local::now().date_naive();

    if today.month() != 7 || today.day() < 4 {
        eprintln!("Today aint the day gng");
        exit(0);
    }

    let errorscr = GtkBox::new(Orientation::Vertical, 10);
    errorscr.set_valign(gtk4::Align::Center);
    errorscr.set_halign(gtk4::Align::Center);
    errorscr.set_hexpand(true);
    errorscr.set_vexpand(true);
    errorscr.set_height_request(500);
    errorscr.set_width_request(800);
    errorscr.add_css_class("error");

    let title = Label::builder()
        .label("Linux 6.0.2")
        .css_classes(["title"])
        .build();

    let message = Label::builder()
    .use_markup(true)
    .label("
        An <b>ERROR</b> has occurred. To Continue press <i>RETURN</i>

        Jun 17 21:53:30 scu NetworkManager[667]: &lt;info&gt;  [1781713410.4936] dhcp6 (wlp3s0): state changed no lease
        Jun 17 21:53:30 scu NetworkManager[667]: &lt;info&gt;  [1781713410.4936] dhcp6 (wlp3s0): activation: beginning transaction (timeout in 45 seconds)
        Jun 17 21:53:30 scu wpa_supplicant[1106]: wlp3s0: CTRL-EVENT-DISCONNECTED bssid=f6:e3:9a:98:9b:bd reason=4 locally_generated=1
        Jun 17 21:53:30 scu wpa_supplicant[1106]: wlp3s0: Added BSSID f6:e3:9a:98:9b:bd into ignore list, ignoring for 10 seconds
        Jun 17 21:53:30 scu wpa_supplicant[1106]: wlp3s0: CTRL-EVENT-REGDOM-CHANGE init=CORE type=WORLD
        Jun 17 21:53:30 scu wpa_supplicant[1106]: wlp3s0: CTRL-EVENT-REGDOM-CHANGE init=CORE type=WORLD
        Jun 17 21:53:30 scu NetworkManager[667]: &lt;info&gt;  [1781713410.8814] device (wlp3s0): supplicant interface state: completed -&gt; disconnected
        Jun 17 21:53:30 scu NetworkManager[667]: &lt;info&gt;  [1781713410.8814] device (p2p-dev-wlp3s0): supplicant management interface state: 
        completed -&gt; disconnected
        Jun 17 21:53:30 scu NetworkManager[667]: &lt;info&gt;  [1781713410.9525] device (wlp3s0): supplicant interface state: disconnected -&gt; scanning
        Jun 17 21:53:30 scu NetworkManager[667]: &lt;info&gt;  [1781713410.9525] device (p2p-dev-wlp3s0): supplicant management interface state: 
        disconnected -&gt; scanning
    ")
    .build();

    errorscr.append(&title);
    errorscr.append(&message);

    let pageone = GtkBox::builder()
        .vexpand(true)
        .hexpand(true)
        .valign(gtk4::Align::Fill)
        .orientation(Orientation::Vertical)
        .halign(gtk4::Align::Fill)
        .css_classes(["pgone"])
        .spacing(40)
        .visible(false)
        .build();

    let jk = Label::builder()
        .label("Just Kidding !!")
        .justify(gtk4::Justification::Left)
        .hexpand(true)
        .vexpand(true)
        .css_classes(["jk"])
        .build();

    let btext = Label::builder()
        .use_markup(true)
        .label("
             ▄▄   ▄▄ ▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄   ▄▄    ▄▄▄▄▄▄▄ ▄▄▄ ▄▄▄▄▄▄   ▄▄▄▄▄▄▄ ▄▄   ▄▄ ▄▄▄▄▄▄  ▄▄▄▄▄▄ ▄▄   ▄▄ ▄▄   ▄▄ ▄▄   ▄▄ ▄▄   ▄▄ 
            █  █ █  █      █       █       █  █ █  █  █  ▄    █   █   ▄  █ █       █  █ █  █      ██      █  █ █  █  █ █  █  █ █  █  █ █  █
            █  █▄█  █  ▄   █    ▄  █    ▄  █  █▄█  █  █ █▄█   █   █  █ █ █ █▄     ▄█  █▄█  █  ▄    █  ▄   █  █▄█  █  █▄█  █  █▄█  █  █▄█  █
            █       █ █▄█  █   █▄█ █   █▄█ █       █  █       █   █   █▄▄█▄  █   █ █       █ █ █   █ █▄█  █       █       █       █       █
            █   ▄   █      █    ▄▄▄█    ▄▄▄█▄     ▄█  █  ▄   ██   █    ▄▄  █ █   █ █   ▄   █ █▄█   █      █▄     ▄█▄     ▄█▄     ▄█▄     ▄█
            █  █ █  █  ▄   █   █   █   █     █   █    █ █▄█   █   █   █  █ █ █   █ █  █ █  █       █  ▄   █ █   █   █   █   █   █   █   █  
            █▄▄█ █▄▄█▄█ █▄▄█▄▄▄█   █▄▄▄█     █▄▄▄█    █▄▄▄▄▄▄▄█▄▄▄█▄▄▄█  █▄█ █▄▄▄█ █▄▄█ █▄▄█▄▄▄▄▄▄██▄█ █▄▄█ █▄▄▄█   █▄▄▄█   █▄▄▄█   █▄▄▄█  


             ▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄ 
            █  ▄    █  ▄    █       █
            █ █▄█   █ █▄█   █   ▄▄▄▄█
            █       █       █  █  ▄▄ 
            █  ▄   ██  ▄   ██  █ █  █
            █ █▄█   █ █▄█   █  █▄▄█ █
            █▄▄▄▄▄▄▄█▄▄▄▄▄▄▄█▄▄▄▄▄▄▄█
        ")
        .hexpand(true)
        .vexpand(true)
        .css_classes(["bday"])
        .build();

    let bd_box = GtkBox::builder()
        .hexpand(true)
        .vexpand(true)
        .css_classes(["bdaybox"])
        .valign(gtk4::Align::End)
        .halign(gtk4::Align::Center)
        .visible(false)
        .build();

    bd_box.append(&btext);

    let start = Label::builder()
        .use_markup(true)
        .label("USE ARROW KEYS")
        .hexpand(true)
        .vexpand(true)
        .valign(gtk4::Align::Start)
        .halign(gtk4::Align::Center)
        .css_classes(["exp"])
        .visible(false)
        .build();

    pageone.append(&jk);
    pageone.append(&bd_box);
    pageone.append(&start);

    let jk_clone = jk.clone();
    let btext_clone = bd_box.clone();
    let start = start.clone();
    let hbd = move|| {
        jk_clone.set_visible(false);
        btext_clone.set_visible(true);
        start.set_visible(true);
    };

    let main = GtkBox::new(Orientation::Vertical, 0);
    main.set_hexpand(true);
    main.set_vexpand(true);
    main.set_css_classes(&["mainwin"]);
    main.set_cursor_from_name(Some(&"none"));


    main.append(&errorscr);
    main.append(&pageone);

    let controller = gtk4::EventControllerKey::new();

    let errorscr_clone = errorscr.clone();
    let pg_clone = pageone.clone();
    controller.connect_key_pressed(move |_controller, keyval, _keycode, _state| {
        if keyval == gtk4::gdk::Key::Return || keyval == gtk4::gdk::Key::KP_Enter {
            errorscr_clone.set_visible(false);
            pg_clone.set_visible(true);
            // std::thread::sleep(Duration::from_secs(2));
            let hbd = hbd.clone();
            gtk4::glib::timeout_add_local(Duration::from_secs(2), move || {
                hbd();
                gtk4::glib::ControlFlow::Break
            });
            gtk4::glib::Propagation::Stop
        } else if keyval == gtk4::gdk::Key::Alt_R {
            exit(0);
        } else if keyval == gtk4::gdk::Key::downarrow {
            
            gtk4::glib::Propagation::Stop
        } else {
            gtk4::glib::Propagation::Proceed
        }
    });

    window.add_controller(controller);
    window.set_child(Some(&main)); 

    window.show();

}
