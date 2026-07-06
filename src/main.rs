use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box as GtkBox, Orientation, Button, Label, CssProvider, Overlay, Picture};
use gtk4::gdk::{Display, Texture};
use gtk4_layer_shell::{LayerShell, Layer, Edge};
use gtk4::gdk_pixbuf::PixbufLoader;
use std::process::exit;
use std::{time::Duration};
use chrono::{Datelike, Local};

mod images;

fn main() {
    let app = Application::builder()
        .application_id("ekah.scu.pager")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

fn animate_switch(
    outgoing: GtkBox,
    outgoing_exit_class: &'static str,
    incoming: GtkBox,
    incoming_enter_class: &'static str,
) {
    outgoing.remove_css_class(outgoing_exit_class);
    incoming.remove_css_class(incoming_enter_class);
    incoming.set_visible(true);

    let outgoing_for_tick = outgoing.clone();
    incoming.add_tick_callback(move |incoming_widget, _clock| {
        outgoing_for_tick.add_css_class(outgoing_exit_class);
        incoming_widget.add_css_class(incoming_enter_class);
        gtk4::glib::ControlFlow::Break
    });

    let outgoing_cleanup = outgoing;
    let incoming_cleanup = incoming;
    gtk4::glib::timeout_add_local(Duration::from_millis(420), move || {
        outgoing_cleanup.set_visible(false);
        outgoing_cleanup.remove_css_class(outgoing_exit_class);
        incoming_cleanup.remove_css_class(incoming_enter_class);
        gtk4::glib::ControlFlow::Break
    });
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
            background-color: transparent;
        }

        .pgtwo {
            background-color: #1d4fd800;
        }

        .pgthree {
            background-color: #80808000;
        }

        .enter-bottom {
            animation: enterFromBottom 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
        }

        @keyframes enterFromBottom {
            from {rgb(255, 255, 255)rgba(255, 255, 255, 0)
                transform: translate(0px, 2000px);
            }
            to {
                transform: translate(0px, 0px);
            }
        }

        .exit-top {
            animation: exitToTop 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
        }

        @keyframes exitToTop {
            from {
                transform: translate(0px, 0px);
            }
            to {
                transform: translate(0px, -2000px);
            }
        }

        .enter-top {
            animation: enterFromTop 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
        }

        @keyframes enterFromTop {
            from {
                transform: translate(0px, -2000px);
            }
            to {
                transform: translate(0px, 0px);
            }
        }

        .exit-bottom {
            animation: exitToBottom 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
        }

        @keyframes exitToBottom {
            from {
                transform: translate(0px, 0px);
            }
            to {
                transform: translate(0px, 2000px);
            }
        }

        .exit-left {
            animation: exitToLeft 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
        }

        @keyframes exitToLeft {
            from {
                transform: translate(0px, 0px);
            }
            to {
                transform: translate(-2000px, 0px);
            }
        }

        .enter-right {
            animation: enterFromRight 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
        }

        @keyframes enterFromRight {
            from {
                transform: translate(2000px, 0px);
            }
            to {
                transform: translate(0px, 0px);
            }
        }

        .exit-right {
            animation: exitToRight 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
        }

        @keyframes exitToRight {
            from {
                transform: translate(0px, 0px);
            }
            to {
                transform: translate(2000px, 0px);
            }
        }

        .enter-left {
            animation: enterFromLeft 0.4s cubic-bezier(0.16, 1, 0.3, 1) forwards;
        }

        @keyframes enterFromLeft {
            from {
                transform: translate(-2000px, 0px);
            }
            to {
                transform: translate(0px, 0px);
            }
        }

        .page {
            color: black;
            font-family: FreeMono, monospace;
            font-style: italic;
            font-size: 16px;
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

    if today.month() != 7 || today.day() < 7 {
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

    let pagetwo = GtkBox::builder()
        .vexpand(true)
        .hexpand(true)
        .valign(gtk4::Align::Center)
        .orientation(Orientation::Vertical)
        .halign(gtk4::Align::Center)
        .css_classes(["pgtwo"])
        .spacing(40)
        .visible(false)
        .build();

    let pagetext = Label::builder()
        .use_markup(true)
        .label("<b>         Happy birthday my clumsy nigga babyyyyyy!!!</b>
        
This will be you'r 3rd birthday we be spending together,
just remindes me how long we've been dating and how much
fun its been growing three more years older with you.
    
All this time from 11th Jan 2024, we've changed a lot
and also made a lot of memories together.. I HAVE LOVED 
ALLL THE TIME I HAVE SPENT WITH YOU.

My baby penglin I want to grow older with u, seeing u do 
the most stupidest cutest shi and be supportive in all 
ur dreams. I want to be with u for all the changes in our
lives. muahhh (if u havn't gone right key yet go back up
and go right ;) )
        ")
        .hexpand(true)
        .vexpand(true)
        .halign(gtk4::Align::Center)
        .valign(gtk4::Align::Center)
        .width_request(400)
        .height_request(800)
        .css_classes(["page"])
        .build();

    let frogyloader = PixbufLoader::new();
    frogyloader
        .write(images::FROGGY.as_bytes())
        .expect("Failed to write SVG data to PixbufLoader");
    frogyloader
        .close()
        .expect("Failed to close PixbufLoader (invalid SVG?)");
    let froggy_pix = frogyloader
        .pixbuf()
        .expect("froggy_pixLoader produced no froggy_pix from SVG");
    let froggy_tex = Texture::for_pixbuf(&froggy_pix);
    let frogy = Picture::builder()
        .paintable(&froggy_tex)
        .hexpand(true)
        .vexpand(true)
        .width_request(200)
        .height_request(200)
        .build();

    let ovalay = Overlay::builder()
        .margin_top(50)
        .margin_bottom(50)
        .margin_start(50)
        .margin_end(50)
        .build();

    ovalay.set_child(Some(&frogy));
    ovalay.add_overlay(&pagetext);

    pagetwo.append(&ovalay);

    let pagethree = GtkBox::builder()
        .vexpand(true)
        .hexpand(true)
        .valign(gtk4::Align::Fill)
        .orientation(Orientation::Vertical)
        .halign(gtk4::Align::Fill)
        .css_classes(["pgthree"])
        .visible(false)
        .build();

    let collyloader = PixbufLoader::new();
    collyloader
        .write(images::COLLY.as_bytes())
        .expect("Failed to write SVG data to PixbufLoader");
    collyloader
        .close()
        .expect("Failed to close PixbufLoader (invalid SVG?)");
    let colly_pix = collyloader
        .pixbuf()
        .expect("colly_pixLoader produced no colly_pix from SVG");
    let colly_tex = Texture::for_pixbuf(&colly_pix);
    let colly = Picture::builder()
        .paintable(&colly_tex)
        .hexpand(true)
        .vexpand(true)
        .build();

    pagethree.append(&colly);

    let main = GtkBox::new(Orientation::Vertical, 0);
    main.set_hexpand(true);
    main.set_vexpand(true);
    main.set_css_classes(&["mainwin"]);
    main.set_cursor_from_name(Some(&"none"));


    let pages_overlay = Overlay::new();
    pages_overlay.set_hexpand(true);
    pages_overlay.set_vexpand(true);
    pages_overlay.set_child(Some(&pageone));
    pages_overlay.add_overlay(&pagetwo);
    pages_overlay.add_overlay(&pagethree);
    pages_overlay.set_visible(false);

    main.append(&errorscr);
    main.append(&pages_overlay);

    let controller = gtk4::EventControllerKey::new();

    let errorscr_clone = errorscr.clone();
    let pg_clone = pageone.clone();
    let pgt = pagetwo.clone();
    let pgth = pagethree.clone();
    let pages_overlay_clone = pages_overlay.clone();
    controller.connect_key_pressed(move |_controller, keyval, _keycode, _state| {
        if keyval == gtk4::gdk::Key::Return || keyval == gtk4::gdk::Key::KP_Enter {
            errorscr_clone.set_visible(false);
            pages_overlay_clone.set_visible(true);
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
        } else if keyval == gtk4::gdk::Key::Down {
            if pg_clone.get_visible() {
                animate_switch(pg_clone.clone(), "exit-top", pgt.clone(), "enter-bottom");
            } else {
                eprintln!("bodnwqnewkic");
            }
            gtk4::glib::Propagation::Stop
        } else if keyval == gtk4::gdk::Key::Up {
            if pgt.get_visible() {
                animate_switch(pgt.clone(), "exit-bottom", pg_clone.clone(), "enter-top");
            } else {
                eprintln!("bodnwqnewkic");
            }
            gtk4::glib::Propagation::Stop
        } else if keyval == gtk4::gdk::Key::Right {
            if pg_clone.get_visible() {
                animate_switch(pg_clone.clone(), "exit-left", pgth.clone(), "enter-right");
            } else {
                eprintln!("bodnwqnewkic");
            }
            gtk4::glib::Propagation::Stop
        } else if keyval == gtk4::gdk::Key::Left {
            if pgth.get_visible() {
                animate_switch(pgth.clone(), "exit-right", pg_clone.clone(), "enter-left");
            } else {
                eprintln!("bodnwqnewkic");
            }
            gtk4::glib::Propagation::Stop
        } else {
            gtk4::glib::Propagation::Proceed
        }
    });

    window.add_controller(controller);
    window.set_child(Some(&main)); 

    window.show();

}