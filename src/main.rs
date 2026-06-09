/*
 * ============================================================================
 *  Project     : dashboard-core-drag-drop
 *  Description : 可変オブジェクト型・動的ダッシュボード
 *  Language    : Rust 🦀 & Slint (Declarative GUI UI v1.0+)
 *  License     : MIT License
 *  Author      : Kenji Igarashi
 *  GitHub      : https://github.com/kenjiigarashi
 *  LinkedIn    : https://www.linkedin.com/in/kenji-igarashi-123456789/
 *  License     : MIT License
 *  Created     : 2026-06-03
 *  Updated     : 
 * ============================================================================
 *  [Notice]
 *  本システム内に実装されている各機能（資産残高、時計、日付、経済指標など）の
 *  内部ロジックは、フレームワークとしての挙動を検証するためのサンプル（モック）です。
 *  用途に合わせて、独自の専門コンポーネントへ自由に入れ替えて拡張してください。
 *  [Architectural Design Philosophy]
 *  本システムは、Rustの安全性とSlintの宣言型UIを最大限に活かし、完全なステート駆動型で構築された
 *  ダッシュボードフレームワークです。全てのUIコンポーネントは、共通の状態管理構造体(ComponentState)
 *  として構築された、ステート駆動（宣言型UI）のカスタム基盤です。
 * ===========================================================================
 */
use slint::Model;

slint::slint! {
    import { VerticalBox, HorizontalBox, Button, LineEdit, ScrollView } from "std-widgets.slint";
    import { AboutSlint } from "std-widgets.slint";

    export struct ComponentState {
        id: int,
        title: string,
        comp-type: string,
        is-expanded: bool,
        is-visible: bool,
        offset-y: length,
    }

    // ─── 【自作D&Dフレーム：タップ移動モード版】 ───
    component CustomWidgetFrame inherits Rectangle {
        in property <string> title: "機能パーツ";
        in property <bool> is-expanded: true;
        in property <bool> is-selected: false;
        in property <int> index: 0;
        in property <length> offset-y: 0px;

        // タップされた瞬間（移動モード開始）を親に伝える窓口
        callback handle-clicked(int, length);

        callback toggle-size(); callback move-up(); callback move-down();
        callback hide-component();
        callback clear-selection();

        // 自由移動用のY座標
        in-out property <length> pos-y: root.offset-y;
        in property <bool> is-ghost: false;

        // 移動モード中でない通常時だけ、150msの滑らかな吸着アニメーションを発動！
        animate pos-y { duration: root.is-selected ? 0ms : 150ms; easing: ease-in-out; }

        changed offset-y => {
            if (!root.is-selected) {
                root.pos-y = root.offset-y;
            }
        }

        y: root.pos-y;
        x: 0px;
        width: root.is-expanded ? 440px : 210px;
        height: root.is-expanded ? 120px : 80px;

        // ゴースト空席のときは超薄い灰色に、移動モード中は上品な透明ブルーグレーに！
        background: root.is-ghost ? #00000007 : (root.is-selected ? #eaf2f8 : #ffffff);
        border-radius: 8px; 
        border-width: 2px;
        border-color: root.is-ghost ? #bdc3c733 : (root.is-selected ? #3498db : #bdc3c7);
        drop-shadow-blur: root.is-selected ? 15px : 0px;
        drop-shadow-color: #0000002b;

        HorizontalBox {
            padding: 4px; spacing: 6px;
            visible: !root.is-ghost;

            Rectangle {
                width: 35px;
                background: root.is-selected ? #3498db : #f0f3f6; border-radius: 4px;
                
                TouchArea { 
                    clicked => {
                        // 掴んで「タップ」したら、移動モードをONにして現在位置を親に登録！
                        root.handle-clicked(root.index, root.offset-y);
                    }
                }
                
                Text { 
                    text: root.is-selected ? "OK" : "::"; 
                    font-size: 14px; 
                    font-weight: 700; 
                    color: root.is-selected ? #ffffff : #7f8c8d; 
                    horizontal-alignment: center; 
                    vertical-alignment: center; 
                }
            }
            
            VerticalBox {
                padding: 4px; spacing: 4px;
                HorizontalBox {
                    padding: 0;
                    Text { text: root.title; font-weight: 700; font-size: 13px; color: #2c3e50; vertical-alignment: center; }
                    HorizontalBox {
                        alignment: end; spacing: 3px; padding: 0;
                        Button { text: "↑"; width: 25px; clicked => { root.move-up() } }
                        Button { text: "↓"; width: 25px; clicked => { root.move-down() } }
                        Button { text: root.is-expanded ? "縮" : "広"; width: 30px; clicked => { root.toggle-size() } }
                        Button { text: "X"; width: 25px; clicked => { root.hide-component() } }
                    }
                }
                @children
            }
        }
    }

    // 各専用コンテンツ
    component ZandakaComp inherits VerticalBox { Text { text: "現在の想定総残高: ¥4,567,890"; font-size: 15px; color: #27ae60; font-weight: 700; } }
    component ClockComp inherits VerticalBox { in property <string> time; Text { text: root.time; font-size: 18px; color: #2980b9; font-weight: 700; } }
    component BoxComp inherits VerticalBox { in property <string> date; Text { text: root.date; font-size: 14px; color: #7f8c8d; } }
    component MemoComp inherits VerticalBox { LineEdit { placeholder-text: "重要メモをここに書き留める..."; } }
    component AdComp inherits VerticalBox { Rectangle { background: #fdf2e9; Text { text: "[広告枠] 最新のRust学習書、好評発売中！"; color: #e67e22; font-size: 11px; } } }
    component TenkiComp inherits VerticalBox { HorizontalBox { alignment: start; spacing: 10px; Text { text: "Fine"; font-size: 18px; font-weight: 700; color: #e67e22; } VerticalBox { Text { text: "東京の天気: 晴れ"; font-size: 14px; font-weight: 700; } Text { text: "気温: 26度 / 降水確率: 10%"; font-size: 11px; color: #7f8c8d; } } } }
    component KabukaComp inherits VerticalBox { Text { text: "日経平均株価: ¥38,500 (+250)"; font-size: 14px; color: #e74c3c; font-weight: 700; } HorizontalBox { spacing: 5px; alignment: start; Rectangle { background: #e74c3c; width: 20px; height: 10px; } Rectangle { background: #e74c3c; width: 20px; height: 20px; } Rectangle { background: #e74c3c; width: 20px; height: 35px; } } }
    component TodoComp inherits VerticalBox { spacing: 3px; Text { text: "[ ] 1. 経費精算を終わらせる"; font-size: 12px; } Text { text: "[ ] 2. 15時からミーティング"; font-size: 12px; } Text { text: "[V] 3. Slintアプリの基盤設計を考える"; font-size: 12px; color: #95a5a6; } }
    // ─── ③ メイン画面 ───
    export component AppWindow inherits Window {
        title: "カスタムダッシュボード基盤";
        width: 470px; height: 650px; background: #f4f6f7;
        in-out property <[ComponentState]> comp-list: [];
        in property <string> current-time-str: "19:05:00";
        in property <string> current-date-str: "2026年06月03日";
        
        in-out property <int> selected-idx: -1; 
        in property <length> total-scroll-height: 600px;

        // 移動同期用の絶対座標管理プロパティ群
        in-out property <length> active-drag-y: 0px;
        in-out property <length> drag-global-start-y: 0px;
        in-out property <length> drag-item-start-y: 0px;
        in-out property <ComponentState> active-drag-item;

        callback cmd-toggle-size(int); callback cmd-move(int, bool);
        callback cmd-hide(int); callback cmd-add-next(); callback cmd-select-handle(int);
        callback cmd-reorder-dnd(int, length);

        // 画面全体を包み込む親空間にして、最前面にオーバーレイを重ねられる構造
        Rectangle {
            width: 100%; height: 100%;

            ScrollView {
                width: 100%; 
                height: 100%; 

                VerticalBox {
                    padding: 10px; spacing: 10px;
                    
                    Rectangle {
                        width: 440px;
                        height: root.total-scroll-height;
                        background: transparent;

                        // 【レイヤー1】：通常のリスト空間
                        for state[idx] in root.comp-list : CustomWidgetFrame {
                            title: state.title;
                            is-expanded: state.is-expanded;
                            is-selected: root.selected-idx == idx; 
                            index: idx;
                            offset-y: state.offset-y;
                            is-ghost: root.selected-idx == idx;

                            toggle-size => { root.cmd-toggle-size(idx); }
                            move-up => { root.cmd-move(idx, true); }
                            move-down => { root.cmd-move(idx, false); }
                            hide-component => { root.cmd-hide(idx); }
                            
                            handle-clicked(from_idx, start_pos_y) => {
                                root.cmd-select-handle(from_idx);
                                root.active-drag-y = start_pos_y;
                                root.drag-item-start-y = start_pos_y;
                                root.active-drag-item = state;
                            }
                            clear-selection => { root.selected-idx = -1; }

                            if state.comp-type == "zandaka" : ZandakaComp {}
                            if state.comp-type == "clock" : ClockComp { time: root.current-time-str; }
                            if state.comp-type == "date" : BoxComp { date: root.current-date-str; }
                            if state.comp-type == "memo" : MemoComp {}
                            if state.comp-type == "ad" : AdComp {}
                            if state.comp-type == "tenki" : TenkiComp {}
                            if state.comp-type == "kabuka" : KabukaComp {}
                            if state.comp-type == "todo" : TodoComp {}
                        }

                        // 【レイヤー2】：移動モード中だけ出現する「完全最前面浮遊シアター」
                        if (root.selected-idx != -1) : Rectangle {
                            y: root.active-drag-y;
                            x: 0px;
                            width: root.active-drag-item.is-expanded ? 440px : 210px;
                            height: root.active-drag-item.is-expanded ? 120px : 80px;
                            background: #3498db33;
                            border-radius: 8px; border-width: 2px; border-color: #3498db;
                            drop-shadow-blur: 15px; drop-shadow-color: #0000002b;

                            HorizontalBox {
                                padding: 4px; spacing: 6px;
                                Rectangle {
                                    width: 35px; background: #3498db; border-radius: 4px;
                                    Text { text: "OK"; font-size: 13px; font-weight: 700; color: #ffffff; horizontal-alignment: center; vertical-alignment: center; }
                                }
                                VerticalBox {
                                    padding: 4px; spacing: 4px;
                                    Text { text: root.active-drag-item.title; font-weight: 700; font-size: 13px; color: #2c3e50; }
                                    if root.active-drag-item.comp-type == "zandaka" : ZandakaComp {}
                                    if root.active-drag-item.comp-type == "clock" : ClockComp { time: root.current-time-str; }
                                    if root.active-drag-item.comp-type == "date" : BoxComp { date: root.current-date-str; }
                                    if root.active-drag-item.comp-type == "memo" : MemoComp {}
                                    if root.active-drag-item.comp-type == "ad" : AdComp {}
                                    if root.active-drag-item.comp-type == "tenki" : TenkiComp {}
                                    if root.active-drag-item.comp-type == "kabuka" : KabukaComp {}
                                    if root.active-drag-item.comp-type == "todo" : TodoComp {}
                                }
                            }
                        }

                        // 【レイヤー3】：全画面透過型ドラッグセンサーシールド
                        if (root.selected-idx != -1) : TouchArea {
                            width: 100%; height: 100%;
                            
                            pointer-event(event) => {
                                if (event.kind == PointerEventKind.down) {
                                    root.drag-global-start-y = self.mouse-y;
                                    root.drag-item-start-y = root.active-drag-y;
                                }
                                if (event.kind == PointerEventKind.up) {
                                    root.cmd-reorder-dnd(root.selected-idx, root.active-drag-y + 40px);
                                    root.selected-idx = -1; 
                                }
                            }
                            moved => {
                                if (self.pressed) {
                                    root.active-drag-y = root.drag-item-start-y + (self.mouse-y - root.drag-global-start-y);
                                }
                            }
                        }
                    }

                    HorizontalBox {
                        alignment: space-between;
                        padding-top: 15px;
                        Button { text: "About"; width: 80px; height: 32px; clicked => { about_popup.show(); } }
                        Button { text: "+ 新しい機能を追加"; primary: true; height: 32px; clicked => { root.cmd-add-next(); } }
                    }
                }
            }

            // ─── 最前面のSFシステム警告『MOVE MODE』オーバーレイ！ ───
            if (root.selected-idx != -1) : Rectangle {
                width: 100%; height: 100%;
                background: #3498db08; 
                border-width: 4px; border-color: #3498db88; 

                // 画面上部に未来感あふれる警告バーを表示
                Rectangle {
                    y: 10px; x: (parent.width - self.width) / 2;
                    width: 220px; height: 35px;
                    background: #3498db; border-radius: 20px;
                    drop-shadow-blur: 10px; drop-shadow-color: #3498db88;

                    HorizontalBox {
                        alignment: center; padding: 0;
                        Text {
                            text: "⚠️ MOVE MODE ACTIVE";
                            font-size: 13px; font-weight: 900;
                            color: #ffffff;
                            vertical-alignment: center; horizontal-alignment: center;
                        }
                    }
                }
            }
        }

        about_popup := PopupWindow {
            width: root.width; height: root.height;
            x: 0; y: 0;
            
            Rectangle {
                width: 100%; height: 100%;
                background: transparent;

                Rectangle {
                    width: 300px; height: 240px;
                    x: (parent.width - self.width) * 0.5;
                    y: (parent.height - self.height) * 0.5;
                    background: #67696b5e;
                    border-radius: 4px;
                    
                    VerticalBox {
                        padding: 15px; spacing: 10px;
                        Text { color: #1b2530; font-size: 16px; font-weight: 700; horizontal-alignment: center; text: "App Ver.1.0"; }
                        HorizontalLayout { alignment: center; AboutSlint { width: 200px; height: 80px; } }
                        HorizontalLayout { alignment: center; Button { text: "閉じる"; width: 100px; clicked => { about_popup.close(); } } }
                    }
                }
            }
        }
    }
}
fn update_component_offsets(ui: &AppWindow, model: &std::rc::Rc<slint::VecModel<ComponentState>>) {
    let mut current_y = 0.0;
    let mut new_vec = Vec::new();
    
    for mut item in model.iter() {
        item.offset_y = current_y;
        if item.is_expanded {
            current_y += 130.0;
        } else {
            current_y += 90.0;
        }
        new_vec.push(item);
    }
    
    model.set_vec(new_vec);
    ui.set_total_scroll_height(current_y + 40.0);
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    
    let initial_components = vec![
        ComponentState { id: 1, title: "資産・リアルタイム残高".into(), comp_type: "zandaka".into(), is_expanded: true, is_visible: true, offset_y: 0.0 },
        ComponentState { id: 2, title: "システム時計".into(), comp_type: "clock".into(), is_expanded: false, is_visible: true, offset_y: 0.0 },
        ComponentState { id: 3, title: "今日の日付".into(), comp_type: "date".into(), is_expanded: false, is_visible: true, offset_y: 0.0 },
        ComponentState { id: 4, title: "クイック・ビジネスメモ".into(), comp_type: "memo".into(), is_expanded: true, is_visible: true, offset_y: 0.0 },
        ComponentState { id: 5, title: "スポンサー広告枠".into(), comp_type: "ad".into(), is_expanded: true, is_visible: true, offset_y: 0.0 },
    ];
    
    let model = std::rc::Rc::new(slint::VecModel::from(initial_components));
    ui.set_comp_list(model.clone().into());
    
    ui.set_active_drag_item(ComponentState {
        id: 0,
        title: "".into(),
        comp_type: "".into(),
        is_expanded: false,
        is_visible: false,
        offset_y: 0.0,
    });
    
    update_component_offsets(&ui, &model);

    let model_clone = model.clone();
    let ui_handle = ui.as_weak();
    ui.on_cmd_toggle_size(move |idx| {
        if let (Some(ui_instance), Some(mut data)) = (ui_handle.upgrade(), model_clone.row_data(idx as usize)) {
            data.is_expanded = !data.is_expanded;
            model_clone.set_row_data(idx as usize, data);
            update_component_offsets(&ui_instance, &model_clone);
        }
    });

    let model_clone = model.clone();
    let ui_handle = ui.as_weak();
    ui.on_cmd_move(move |idx, move_up| {
        if let Some(ui_instance) = ui_handle.upgrade() {
            let i = idx as usize;
            let mut vec: Vec<ComponentState> = model_clone.iter().collect();
            if move_up && i > 0 { vec.swap(i, i - 1); } 
            else if !move_up && i < vec.len() - 1 { vec.swap(i, i + 1); }
            model_clone.set_vec(vec);
            update_component_offsets(&ui_instance, &model_clone);
        }
    });

    let model_clone = model.clone();
    let ui_handle = ui.as_weak();
    ui.on_cmd_hide(move |idx| {
        if let Some(ui_instance) = ui_handle.upgrade() {
            let i = idx as usize;
            let mut vec: Vec<ComponentState> = model_clone.iter().collect();
            if i < vec.len() {
                vec.remove(i); 
                model_clone.set_vec(vec);
                update_component_offsets(&ui_instance, &model_clone);
            }
        }
    });

    let model_clone = model.clone();
    let ui_handle = ui.as_weak();
    ui.on_cmd_add_next(move || {
        if let Some(ui_instance) = ui_handle.upgrade() {
            let vec: Vec<ComponentState> = model_clone.iter().collect();
            if let Some(last_item) = vec.last() {
                let last_type = last_item.comp_type.as_str();
                if last_type == "ad" {
                    model_clone.push(ComponentState { id: 6, title: "お天気ウェザー".into(), comp_type: "tenki".into(), is_expanded: true, is_visible: true, offset_y: 0.0 });
                } else if last_type == "tenki" {
                    model_clone.push(ComponentState { id: 7, title: "ビジネス経済指標".into(), comp_type: "kabuka".into(), is_expanded: true, is_visible: true, offset_y: 0.0 });
                } else if last_type == "kabuka" {
                    model_clone.push(ComponentState { id: 8, title: "本日のタスク(Todo)".into(), comp_type: "todo".into(), is_expanded: true, is_visible: true, offset_y: 0.0 });
                }
                update_component_offsets(&ui_instance, &model_clone);
            }
        }
    });

    // 💡 所有権＆ワーニング大補正：使っていなかった未使用変数（394行目の model_clone）を完全に綺麗さっぱり消去したのら！
    let ui_select_handle = ui.as_weak();
    ui.on_cmd_select_handle(move |idx| {
        if let Some(ui_instance) = ui_select_handle.upgrade() {
            ui_instance.set_selected_idx(idx as i32);
        }
    });

    let model_clone = model.clone();
    let ui_reorder_handle = ui.as_weak();
    ui.on_cmd_reorder_dnd(move |from_idx, drop_y| {
        if let Some(ui_instance) = ui_reorder_handle.upgrade() {
            let from = from_idx as usize;
            let mut to = 0;
            let vec: Vec<ComponentState> = model_clone.iter().collect();
            
            for (i, item) in vec.iter().enumerate() {
                if drop_y > item.offset_y {
                    to = i;
                }
            }

            if from < vec.len() && to < vec.len() && from != to {
                let mut new_vec = vec;
                let item = new_vec.remove(from);
                new_vec.insert(to, item);
                model_clone.set_vec(new_vec);
                update_component_offsets(&ui_instance, &model_clone);
            } else {
                update_component_offsets(&ui_instance, &model_clone);
            }
        }
    });

    let ui_timer_handle = ui.as_weak();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
            let now = std::time::SystemTime::now();
            let since_the_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap_or_default();
            let total_seconds = since_the_epoch.as_secs();
            
            let jst_seconds = total_seconds + (9 * 3600); 
            let secs = jst_seconds % 60;
            let mins = (jst_seconds / 60) % 60;
            let hours = (jst_seconds / 3600) % 24;
            let current_time_string = format!("{:02}:{:02}:{:02}", hours, mins, secs);
            
            let ui_handle_for_loop = ui_timer_handle.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui_instance) = ui_handle_for_loop.upgrade() {
                    ui_instance.set_current_time_str(current_time_string.into());
                }
            });
        }
    });

    ui.set_current_time_str("00:00:00".into());
    ui.set_current_date_str("2026年06月06日".into());

    ui.run()
}
