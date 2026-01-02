use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    webview::WebviewWindowBuilder,
    Manager, WebviewUrl, WindowEvent,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

const TITLEBAR_SCRIPT: &str = r#"
(function() {
    'use strict';

    function waitForTauri(callback, maxAttempts = 100) {
        let attempts = 0;
        function check() {
            attempts++;
            if (window.__TAURI_INTERNALS__ && window.__TAURI_INTERNALS__.invoke) {
                callback(window.__TAURI_INTERNALS__.invoke);
            } else if (attempts < maxAttempts) {
                setTimeout(check, 50);
            } else {
                console.error('Tauri internals not available');
            }
        }
        check();
    }

    function createTitlebar(invoke) {
        if (document.getElementById('custom-titlebar')) return;
        if (document.getElementById('custom-titlebar-spacer')) return;

        const style = document.createElement('style');
        style.id = 'custom-titlebar-style';
        style.textContent = `
            #custom-titlebar {
                position: fixed !important;
                top: 0 !important;
                left: 0 !important;
                right: 0 !important;
                height: 38px !important;
                background: linear-gradient(180deg, #1a1b1b 0%, #151616 100%) !important;
                display: flex !important;
                align-items: center !important;
                justify-content: space-between !important;
                padding: 0 12px !important;
                z-index: 2147483647 !important;
                user-select: none !important;
                -webkit-user-select: none !important;
                border-bottom: 1px solid #0a0a0a !important;
                font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif !important;
                box-sizing: border-box !important;
            }
            #custom-titlebar * {
                box-sizing: border-box !important;
            }
            #custom-titlebar .tb-title-section {
                display: flex !important;
                align-items: center !important;
                gap: 10px !important;
                flex: 1 !important;
                height: 100% !important;
                cursor: grab !important;
            }
            #custom-titlebar .tb-title-section:active {
                cursor: grabbing !important;
            }
            #custom-titlebar .tb-title-icon {
                width: 22px !important;
                height: 22px !important;
                min-width: 22px !important;
                border-radius: 5px !important;
                background: linear-gradient(135deg, #20b2aa 0%, #1a8a84 100%) !important;
                display: flex !important;
                align-items: center !important;
                justify-content: center !important;
                font-weight: 600 !important;
                color: white !important;
                font-size: 13px !important;
            }
            #custom-titlebar .tb-title-text {
                color: #b0b0b0 !important;
                font-size: 13px !important;
                font-weight: 500 !important;
            }
            #custom-titlebar .tb-window-controls {
                display: flex !important;
                gap: 8px !important;
            }
            #custom-titlebar .tb-win-btn {
                width: 13px !important;
                height: 13px !important;
                min-width: 13px !important;
                border-radius: 50% !important;
                border: none !important;
                cursor: pointer !important;
                display: flex !important;
                align-items: center !important;
                justify-content: center !important;
                transition: filter 0.15s !important;
                padding: 0 !important;
                margin: 0 !important;
                outline: none !important;
            }
            #custom-titlebar .tb-win-btn:hover {
                filter: brightness(1.15) !important;
            }
            #custom-titlebar .tb-win-btn svg {
                width: 7px !important;
                height: 7px !important;
                opacity: 0 !important;
                transition: opacity 0.15s !important;
                pointer-events: none !important;
            }
            #custom-titlebar .tb-window-controls:hover .tb-win-btn svg {
                opacity: 1 !important;
            }
            #custom-titlebar .tb-btn-minimize { background: #febc2e !important; }
            #custom-titlebar .tb-btn-minimize svg { stroke: #9a6a00 !important; }
            #custom-titlebar .tb-btn-maximize { background: #28c840 !important; }
            #custom-titlebar .tb-btn-maximize svg { stroke: #0a6518 !important; }
            #custom-titlebar .tb-btn-close { background: #ff5f57 !important; }
            #custom-titlebar .tb-btn-close svg { stroke: #820005 !important; }
        `;

        (document.head || document.documentElement).appendChild(style);

        const titlebar = document.createElement('div');
        titlebar.id = 'custom-titlebar';
        titlebar.innerHTML = `
            <div class="tb-title-section" id="tb-drag-region">
                <div class="tb-title-icon">P</div>
                <span class="tb-title-text">Perplexity</span>
            </div>
            <div class="tb-window-controls">
                <button class="tb-win-btn tb-btn-minimize" id="tb-btn-minimize" title="Minimize">
                    <svg viewBox="0 0 10 10" fill="none" stroke-width="2"><line x1="1" y1="5" x2="9" y2="5"/></svg>
                </button>
                <button class="tb-win-btn tb-btn-maximize" id="tb-btn-maximize" title="Maximize">
                    <svg viewBox="0 0 10 10" fill="none" stroke-width="1.5"><rect x="1" y="1" width="8" height="8" rx="1"/></svg>
                </button>
                <button class="tb-win-btn tb-btn-close" id="tb-btn-close" title="Close">
                    <svg viewBox="0 0 10 10" fill="none" stroke-width="2"><line x1="1" y1="1" x2="9" y2="9"/><line x1="9" y1="1" x2="1" y2="9"/></svg>
                </button>
            </div>
        `;

        // Insert titlebar and spacer
        const spacer = document.createElement('div');
        spacer.id = 'custom-titlebar-spacer';
        spacer.style.cssText = 'height: 38px !important; width: 100% !important; flex-shrink: 0 !important;';

        if (document.body) {
            document.body.insertBefore(spacer, document.body.firstChild);
            document.body.insertBefore(titlebar, document.body.firstChild);
        } else {
            document.documentElement.appendChild(titlebar);
            document.documentElement.appendChild(spacer);
        }

        // Event handlers
        document.getElementById('tb-btn-minimize').addEventListener('click', function(e) {
            e.preventDefault();
            e.stopPropagation();
            invoke('win_minimize');
        });

        document.getElementById('tb-btn-maximize').addEventListener('click', function(e) {
            e.preventDefault();
            e.stopPropagation();
            invoke('win_toggle_maximize');
        });

        document.getElementById('tb-btn-close').addEventListener('click', function(e) {
            e.preventDefault();
            e.stopPropagation();
            invoke('win_hide');
        });

        document.getElementById('tb-drag-region').addEventListener('mousedown', function(e) {
            if (e.button === 0 && e.detail < 2) {
                invoke('win_start_drag');
            }
        });

        document.getElementById('tb-drag-region').addEventListener('dblclick', function(e) {
            e.preventDefault();
            invoke('win_toggle_maximize');
        });

        console.log('Perplexity titlebar initialized');
    }

    function init() {
        if (document.readyState === 'loading') {
            document.addEventListener('DOMContentLoaded', function() {
                waitForTauri(createTitlebar);
            });
        } else {
            waitForTauri(createTitlebar);
        }
    }

    init();
})();
"#;

#[tauri::command]
fn win_minimize(window: tauri::Window) {
    let _ = window.minimize();
}

#[tauri::command]
fn win_hide(window: tauri::Window) {
    let _ = window.hide();
}

#[tauri::command]
async fn win_toggle_maximize(window: tauri::Window) {
    if window.is_maximized().unwrap_or(false) {
        let _ = window.unmaximize();
    } else {
        let _ = window.maximize();
    }
}

#[tauri::command]
fn win_start_drag(window: tauri::Window) {
    let _ = window.start_dragging();
}

fn toggle_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        if window.is_visible().unwrap_or(false) {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            win_minimize,
            win_hide,
            win_toggle_maximize,
            win_start_drag
        ])
        .setup(|app| {
            // Create the Perplexity window (chromeless with injected titlebar)
            let _perplexity_window = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://www.perplexity.ai".parse().unwrap()),
            )
            .title("Perplexity")
            .inner_size(1200.0, 800.0)
            .min_inner_size(800.0, 600.0)
            .decorations(false)
            .resizable(true)
            .visible(true)
            .initialization_script(TITLEBAR_SCRIPT)
            .build()?;

            // Create tray menu
            let show_hide = MenuItem::with_id(app, "show_hide", "Afficher/Masquer", true, None::<&str>)?;
            let new_conv = MenuItem::with_id(app, "new_conv", "Nouvelle conversation", true, None::<&str>)?;
            let separator = MenuItem::with_id(app, "sep", "─────────────", false, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quitter", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show_hide, &new_conv, &separator, &quit])?;

            // Build tray icon
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("Perplexity")
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show_hide" => {
                        toggle_window(app);
                    }
                    "new_conv" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.eval("window.location.href = 'https://www.perplexity.ai'");
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        toggle_window(app);
                    }
                })
                .build(app)?;

            // Register global shortcut: Super+Shift+P (Alt+Shift+P as fallback)
            let app_handle = app.handle().clone();
            let shortcut = Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyP);

            if let Err(_) = app.global_shortcut().on_shortcut(shortcut, {
                let handle = app_handle.clone();
                move |_app, _shortcut, _event| {
                    toggle_window(&handle);
                }
            }) {
                // Fallback to Alt+Shift+P if Super+Shift+P is taken
                let fallback = Shortcut::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::KeyP);
                let _ = app.global_shortcut().on_shortcut(fallback, move |_app, _shortcut, _event| {
                    toggle_window(&app_handle);
                });
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            // Minimize to tray instead of closing
            if let WindowEvent::CloseRequested { api, .. } = event {
                let _ = window.hide();
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
