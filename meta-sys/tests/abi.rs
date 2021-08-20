// Generated by gir (https://github.com/gtk-rs/gir @ 5bbf6cb)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 8e47c67)
// from mutter-gir-files
// DO NOT EDIT

use meta_sys::*;
use std::mem::{align_of, size_of};
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::path::Path;
use std::process::Command;
use std::str;
use tempfile::Builder;

static PACKAGES: &[&str] = &["libmutter-8"];

#[derive(Clone, Debug)]
struct Compiler {
    pub args: Vec<String>,
}

impl Compiler {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut args = get_var("CC", "cc")?;
        args.push("-Wno-deprecated-declarations".to_owned());
        // For _Generic
        args.push("-std=c11".to_owned());
        // For %z support in printf when using MinGW.
        args.push("-D__USE_MINGW_ANSI_STDIO".to_owned());
        args.extend(get_var("CFLAGS", "")?);
        args.extend(get_var("CPPFLAGS", "")?);
        args.extend(pkg_config_cflags(PACKAGES)?);
        Ok(Self { args })
    }

    pub fn compile(&self, src: &Path, out: &Path) -> Result<(), Box<dyn Error>> {
        let mut cmd = self.to_command();
        cmd.arg(src);
        cmd.arg("-o");
        cmd.arg(out);
        let status = cmd.spawn()?.wait()?;
        if !status.success() {
            return Err(format!("compilation command {:?} failed, {}", &cmd, status).into());
        }
        Ok(())
    }

    fn to_command(&self) -> Command {
        let mut cmd = Command::new(&self.args[0]);
        cmd.args(&self.args[1..]);
        cmd
    }
}

fn get_var(name: &str, default: &str) -> Result<Vec<String>, Box<dyn Error>> {
    match env::var(name) {
        Ok(value) => Ok(shell_words::split(&value)?),
        Err(env::VarError::NotPresent) => Ok(shell_words::split(default)?),
        Err(err) => Err(format!("{} {}", name, err).into()),
    }
}

fn pkg_config_cflags(packages: &[&str]) -> Result<Vec<String>, Box<dyn Error>> {
    if packages.is_empty() {
        return Ok(Vec::new());
    }
    let pkg_config = env::var_os("PKG_CONFIG")
        .unwrap_or_else(|| OsString::from("pkg-config"));
    let mut cmd = Command::new(pkg_config);
    cmd.arg("--cflags");
    cmd.args(packages);
    let out = cmd.output()?;
    if !out.status.success() {
        return Err(format!("command {:?} returned {}",
                           &cmd, out.status).into());
    }
    let stdout = str::from_utf8(&out.stdout)?;
    Ok(shell_words::split(stdout.trim())?)
}


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Layout {
    size: usize,
    alignment: usize,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Results {
    /// Number of successfully completed tests.
    passed: usize,
    /// Total number of failed tests (including those that failed to compile).
    failed: usize,
}

impl Results {
    fn record_passed(&mut self) {
        self.passed += 1;
    }
    fn record_failed(&mut self) {
        self.failed += 1;
    }
    fn summary(&self) -> String {
        format!("{} passed; {} failed", self.passed, self.failed)
    }
    fn expect_total_success(&self) {
        if self.failed == 0 {
            println!("OK: {}", self.summary());
        } else {
            panic!("FAILED: {}", self.summary());
        };
    }
}

#[test]
fn cross_validate_constants_with_c() {
    let mut c_constants: Vec<(String, String)> = Vec::new();

    for l in get_c_output("constant").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let value = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse value");
        c_constants.push((name, value));
    }

    let mut results = Results::default();

    for ((rust_name, rust_value), (c_name, c_value)) in
        RUST_CONSTANTS.iter().zip(c_constants.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_value != c_value {
            results.record_failed();
            eprintln!(
                "Constant value mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_value, &c_value
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

#[test]
fn cross_validate_layout_with_c() {
    let mut c_layouts = Vec::new();

    for l in get_c_output("layout").unwrap().lines() {
        let mut words = l.trim().split(';');
        let name = words.next().expect("Failed to parse name").to_owned();
        let size = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse size");
        let alignment = words
            .next()
            .and_then(|s| s.parse().ok())
            .expect("Failed to parse alignment");
        c_layouts.push((name, Layout { size, alignment }));
    }

    let mut results = Results::default();

    for ((rust_name, rust_layout), (c_name, c_layout)) in
        RUST_LAYOUTS.iter().zip(c_layouts.iter())
    {
        if rust_name != c_name {
            results.record_failed();
            eprintln!("Name mismatch:\nRust: {:?}\nC:    {:?}", rust_name, c_name,);
            continue;
        }

        if rust_layout != c_layout {
            results.record_failed();
            eprintln!(
                "Layout mismatch for {}\nRust: {:?}\nC:    {:?}",
                rust_name, rust_layout, &c_layout
            );
            continue;
        }

        results.record_passed();
    }

    results.expect_total_success();
}

fn get_c_output(name: &str) -> Result<String, Box<dyn Error>> {
    let tmpdir = Builder::new().prefix("abi").tempdir()?;
    let exe = tmpdir.path().join(name);
    let c_file = Path::new("tests").join(name).with_extension("c");

    let cc = Compiler::new().expect("configured compiler");
    cc.compile(&c_file, &exe)?;

    let mut abi_cmd = Command::new(exe);
    let output = abi_cmd.output()?;
    if !output.status.success() {
        return Err(format!("command {:?} failed, {:?}", &abi_cmd, &output).into());
    }

    Ok(String::from_utf8(output.stdout)?)
}

const RUST_LAYOUTS: &[(&str, Layout)] = &[
    ("MetaBackend", Layout {size: size_of::<MetaBackend>(), alignment: align_of::<MetaBackend>()}),
    ("MetaBackgroundActorClass", Layout {size: size_of::<MetaBackgroundActorClass>(), alignment: align_of::<MetaBackgroundActorClass>()}),
    ("MetaBackgroundClass", Layout {size: size_of::<MetaBackgroundClass>(), alignment: align_of::<MetaBackgroundClass>()}),
    ("MetaBackgroundContentClass", Layout {size: size_of::<MetaBackgroundContentClass>(), alignment: align_of::<MetaBackgroundContentClass>()}),
    ("MetaBackgroundGroup", Layout {size: size_of::<MetaBackgroundGroup>(), alignment: align_of::<MetaBackgroundGroup>()}),
    ("MetaBackgroundGroupClass", Layout {size: size_of::<MetaBackgroundGroupClass>(), alignment: align_of::<MetaBackgroundGroupClass>()}),
    ("MetaBackgroundImageCacheClass", Layout {size: size_of::<MetaBackgroundImageCacheClass>(), alignment: align_of::<MetaBackgroundImageCacheClass>()}),
    ("MetaBackgroundImageClass", Layout {size: size_of::<MetaBackgroundImageClass>(), alignment: align_of::<MetaBackgroundImageClass>()}),
    ("MetaBarrier", Layout {size: size_of::<MetaBarrier>(), alignment: align_of::<MetaBarrier>()}),
    ("MetaBarrierClass", Layout {size: size_of::<MetaBarrierClass>(), alignment: align_of::<MetaBarrierClass>()}),
    ("MetaBarrierDirection", Layout {size: size_of::<MetaBarrierDirection>(), alignment: align_of::<MetaBarrierDirection>()}),
    ("MetaBarrierEvent", Layout {size: size_of::<MetaBarrierEvent>(), alignment: align_of::<MetaBarrierEvent>()}),
    ("MetaButtonFunction", Layout {size: size_of::<MetaButtonFunction>(), alignment: align_of::<MetaButtonFunction>()}),
    ("MetaButtonLayout", Layout {size: size_of::<MetaButtonLayout>(), alignment: align_of::<MetaButtonLayout>()}),
    ("MetaCloseDialogInterface", Layout {size: size_of::<MetaCloseDialogInterface>(), alignment: align_of::<MetaCloseDialogInterface>()}),
    ("MetaCloseDialogResponse", Layout {size: size_of::<MetaCloseDialogResponse>(), alignment: align_of::<MetaCloseDialogResponse>()}),
    ("MetaCompEffect", Layout {size: size_of::<MetaCompEffect>(), alignment: align_of::<MetaCompEffect>()}),
    ("MetaCompositor", Layout {size: size_of::<MetaCompositor>(), alignment: align_of::<MetaCompositor>()}),
    ("MetaCursor", Layout {size: size_of::<MetaCursor>(), alignment: align_of::<MetaCursor>()}),
    ("MetaCursorTracker", Layout {size: size_of::<MetaCursorTracker>(), alignment: align_of::<MetaCursorTracker>()}),
    ("MetaDebugPaintFlag", Layout {size: size_of::<MetaDebugPaintFlag>(), alignment: align_of::<MetaDebugPaintFlag>()}),
    ("MetaDebugTopic", Layout {size: size_of::<MetaDebugTopic>(), alignment: align_of::<MetaDebugTopic>()}),
    ("MetaDirection", Layout {size: size_of::<MetaDirection>(), alignment: align_of::<MetaDirection>()}),
    ("MetaDisplayCorner", Layout {size: size_of::<MetaDisplayCorner>(), alignment: align_of::<MetaDisplayCorner>()}),
    ("MetaDisplayDirection", Layout {size: size_of::<MetaDisplayDirection>(), alignment: align_of::<MetaDisplayDirection>()}),
    ("MetaDndClass", Layout {size: size_of::<MetaDndClass>(), alignment: align_of::<MetaDndClass>()}),
    ("MetaEdge", Layout {size: size_of::<MetaEdge>(), alignment: align_of::<MetaEdge>()}),
    ("MetaEdgeType", Layout {size: size_of::<MetaEdgeType>(), alignment: align_of::<MetaEdgeType>()}),
    ("MetaExitCode", Layout {size: size_of::<MetaExitCode>(), alignment: align_of::<MetaExitCode>()}),
    ("MetaFrameBorders", Layout {size: size_of::<MetaFrameBorders>(), alignment: align_of::<MetaFrameBorders>()}),
    ("MetaFrameFlags", Layout {size: size_of::<MetaFrameFlags>(), alignment: align_of::<MetaFrameFlags>()}),
    ("MetaFrameType", Layout {size: size_of::<MetaFrameType>(), alignment: align_of::<MetaFrameType>()}),
    ("MetaGrabOp", Layout {size: size_of::<MetaGrabOp>(), alignment: align_of::<MetaGrabOp>()}),
    ("MetaGravity", Layout {size: size_of::<MetaGravity>(), alignment: align_of::<MetaGravity>()}),
    ("MetaInhibitShortcutsDialogInterface", Layout {size: size_of::<MetaInhibitShortcutsDialogInterface>(), alignment: align_of::<MetaInhibitShortcutsDialogInterface>()}),
    ("MetaInhibitShortcutsDialogResponse", Layout {size: size_of::<MetaInhibitShortcutsDialogResponse>(), alignment: align_of::<MetaInhibitShortcutsDialogResponse>()}),
    ("MetaKeyBindingAction", Layout {size: size_of::<MetaKeyBindingAction>(), alignment: align_of::<MetaKeyBindingAction>()}),
    ("MetaKeyBindingFlags", Layout {size: size_of::<MetaKeyBindingFlags>(), alignment: align_of::<MetaKeyBindingFlags>()}),
    ("MetaLaterType", Layout {size: size_of::<MetaLaterType>(), alignment: align_of::<MetaLaterType>()}),
    ("MetaLaunchContextClass", Layout {size: size_of::<MetaLaunchContextClass>(), alignment: align_of::<MetaLaunchContextClass>()}),
    ("MetaLocaleDirection", Layout {size: size_of::<MetaLocaleDirection>(), alignment: align_of::<MetaLocaleDirection>()}),
    ("MetaMaximizeFlags", Layout {size: size_of::<MetaMaximizeFlags>(), alignment: align_of::<MetaMaximizeFlags>()}),
    ("MetaModalOptions", Layout {size: size_of::<MetaModalOptions>(), alignment: align_of::<MetaModalOptions>()}),
    ("MetaMonitorSwitchConfigType", Layout {size: size_of::<MetaMonitorSwitchConfigType>(), alignment: align_of::<MetaMonitorSwitchConfigType>()}),
    ("MetaMotionDirection", Layout {size: size_of::<MetaMotionDirection>(), alignment: align_of::<MetaMotionDirection>()}),
    ("MetaPadActionType", Layout {size: size_of::<MetaPadActionType>(), alignment: align_of::<MetaPadActionType>()}),
    ("MetaPlugin", Layout {size: size_of::<MetaPlugin>(), alignment: align_of::<MetaPlugin>()}),
    ("MetaPluginClass", Layout {size: size_of::<MetaPluginClass>(), alignment: align_of::<MetaPluginClass>()}),
    ("MetaPluginInfo", Layout {size: size_of::<MetaPluginInfo>(), alignment: align_of::<MetaPluginInfo>()}),
    ("MetaPreference", Layout {size: size_of::<MetaPreference>(), alignment: align_of::<MetaPreference>()}),
    ("MetaRectangle", Layout {size: size_of::<MetaRectangle>(), alignment: align_of::<MetaRectangle>()}),
    ("MetaRemoteAccessControllerClass", Layout {size: size_of::<MetaRemoteAccessControllerClass>(), alignment: align_of::<MetaRemoteAccessControllerClass>()}),
    ("MetaRemoteAccessHandle", Layout {size: size_of::<MetaRemoteAccessHandle>(), alignment: align_of::<MetaRemoteAccessHandle>()}),
    ("MetaRemoteAccessHandleClass", Layout {size: size_of::<MetaRemoteAccessHandleClass>(), alignment: align_of::<MetaRemoteAccessHandleClass>()}),
    ("MetaSelectionClass", Layout {size: size_of::<MetaSelectionClass>(), alignment: align_of::<MetaSelectionClass>()}),
    ("MetaSelectionSource", Layout {size: size_of::<MetaSelectionSource>(), alignment: align_of::<MetaSelectionSource>()}),
    ("MetaSelectionSourceClass", Layout {size: size_of::<MetaSelectionSourceClass>(), alignment: align_of::<MetaSelectionSourceClass>()}),
    ("MetaSelectionSourceMemoryClass", Layout {size: size_of::<MetaSelectionSourceMemoryClass>(), alignment: align_of::<MetaSelectionSourceMemoryClass>()}),
    ("MetaSelectionType", Layout {size: size_of::<MetaSelectionType>(), alignment: align_of::<MetaSelectionType>()}),
    ("MetaShadowFactoryClass", Layout {size: size_of::<MetaShadowFactoryClass>(), alignment: align_of::<MetaShadowFactoryClass>()}),
    ("MetaShadowMode", Layout {size: size_of::<MetaShadowMode>(), alignment: align_of::<MetaShadowMode>()}),
    ("MetaShadowParams", Layout {size: size_of::<MetaShadowParams>(), alignment: align_of::<MetaShadowParams>()}),
    ("MetaShapedTextureClass", Layout {size: size_of::<MetaShapedTextureClass>(), alignment: align_of::<MetaShapedTextureClass>()}),
    ("MetaSide", Layout {size: size_of::<MetaSide>(), alignment: align_of::<MetaSide>()}),
    ("MetaSizeChange", Layout {size: size_of::<MetaSizeChange>(), alignment: align_of::<MetaSizeChange>()}),
    ("MetaSoundPlayerClass", Layout {size: size_of::<MetaSoundPlayerClass>(), alignment: align_of::<MetaSoundPlayerClass>()}),
    ("MetaStackLayer", Layout {size: size_of::<MetaStackLayer>(), alignment: align_of::<MetaStackLayer>()}),
    ("MetaStageClass", Layout {size: size_of::<MetaStageClass>(), alignment: align_of::<MetaStageClass>()}),
    ("MetaStartupNotificationClass", Layout {size: size_of::<MetaStartupNotificationClass>(), alignment: align_of::<MetaStartupNotificationClass>()}),
    ("MetaStartupSequence", Layout {size: size_of::<MetaStartupSequence>(), alignment: align_of::<MetaStartupSequence>()}),
    ("MetaStrut", Layout {size: size_of::<MetaStrut>(), alignment: align_of::<MetaStrut>()}),
    ("MetaTabList", Layout {size: size_of::<MetaTabList>(), alignment: align_of::<MetaTabList>()}),
    ("MetaTabShowType", Layout {size: size_of::<MetaTabShowType>(), alignment: align_of::<MetaTabShowType>()}),
    ("MetaVirtualModifier", Layout {size: size_of::<MetaVirtualModifier>(), alignment: align_of::<MetaVirtualModifier>()}),
    ("MetaWaylandClientClass", Layout {size: size_of::<MetaWaylandClientClass>(), alignment: align_of::<MetaWaylandClientClass>()}),
    ("MetaWindowActor", Layout {size: size_of::<MetaWindowActor>(), alignment: align_of::<MetaWindowActor>()}),
    ("MetaWindowClientType", Layout {size: size_of::<MetaWindowClientType>(), alignment: align_of::<MetaWindowClientType>()}),
    ("MetaWindowGroupClass", Layout {size: size_of::<MetaWindowGroupClass>(), alignment: align_of::<MetaWindowGroupClass>()}),
    ("MetaWindowMenuType", Layout {size: size_of::<MetaWindowMenuType>(), alignment: align_of::<MetaWindowMenuType>()}),
    ("MetaWindowType", Layout {size: size_of::<MetaWindowType>(), alignment: align_of::<MetaWindowType>()}),
    ("MetaWorkspaceManagerClass", Layout {size: size_of::<MetaWorkspaceManagerClass>(), alignment: align_of::<MetaWorkspaceManagerClass>()}),
    ("MetaX11DisplayClass", Layout {size: size_of::<MetaX11DisplayClass>(), alignment: align_of::<MetaX11DisplayClass>()}),
];

const RUST_CONSTANTS: &[(&str, &str)] = &[
    ("(guint) META_BARRIER_DIRECTION_NEGATIVE_X", "4"),
    ("(guint) META_BARRIER_DIRECTION_NEGATIVE_Y", "8"),
    ("(guint) META_BARRIER_DIRECTION_POSITIVE_X", "1"),
    ("(guint) META_BARRIER_DIRECTION_POSITIVE_Y", "2"),
    ("(gint) META_BUTTON_FUNCTION_CLOSE", "3"),
    ("(gint) META_BUTTON_FUNCTION_LAST", "4"),
    ("(gint) META_BUTTON_FUNCTION_MAXIMIZE", "2"),
    ("(gint) META_BUTTON_FUNCTION_MENU", "0"),
    ("(gint) META_BUTTON_FUNCTION_MINIMIZE", "1"),
    ("(gint) META_CLOSE_DIALOG_RESPONSE_FORCE_CLOSE", "1"),
    ("(gint) META_CLOSE_DIALOG_RESPONSE_WAIT", "0"),
    ("(gint) META_COMP_EFFECT_CREATE", "0"),
    ("(gint) META_COMP_EFFECT_DESTROY", "2"),
    ("(gint) META_COMP_EFFECT_MINIMIZE", "3"),
    ("(gint) META_COMP_EFFECT_NONE", "4"),
    ("(gint) META_COMP_EFFECT_UNMINIMIZE", "1"),
    ("META_CURRENT_TIME", "0"),
    ("(gint) META_CURSOR_BLANK", "19"),
    ("(gint) META_CURSOR_BUSY", "11"),
    ("(gint) META_CURSOR_CROSSHAIR", "17"),
    ("(gint) META_CURSOR_DEFAULT", "1"),
    ("(gint) META_CURSOR_DND_COPY", "14"),
    ("(gint) META_CURSOR_DND_IN_DRAG", "12"),
    ("(gint) META_CURSOR_DND_MOVE", "13"),
    ("(gint) META_CURSOR_DND_UNSUPPORTED_TARGET", "15"),
    ("(gint) META_CURSOR_EAST_RESIZE", "5"),
    ("(gint) META_CURSOR_IBEAM", "18"),
    ("(gint) META_CURSOR_LAST", "20"),
    ("(gint) META_CURSOR_MOVE_OR_RESIZE_WINDOW", "10"),
    ("(gint) META_CURSOR_NE_RESIZE", "8"),
    ("(gint) META_CURSOR_NONE", "0"),
    ("(gint) META_CURSOR_NORTH_RESIZE", "2"),
    ("(gint) META_CURSOR_NW_RESIZE", "9"),
    ("(gint) META_CURSOR_POINTING_HAND", "16"),
    ("(gint) META_CURSOR_SE_RESIZE", "6"),
    ("(gint) META_CURSOR_SOUTH_RESIZE", "3"),
    ("(gint) META_CURSOR_SW_RESIZE", "7"),
    ("(gint) META_CURSOR_WEST_RESIZE", "4"),
    ("(guint) META_DEBUG_DBUS", "262144"),
    ("(guint) META_DEBUG_EDGE_RESISTANCE", "131072"),
    ("(guint) META_DEBUG_EVENTS", "16"),
    ("(guint) META_DEBUG_FOCUS", "1"),
    ("(guint) META_DEBUG_GEOMETRY", "128"),
    ("(guint) META_DEBUG_GROUPS", "16384"),
    ("(guint) META_DEBUG_INPUT", "524288"),
    ("(guint) META_DEBUG_KEYBINDINGS", "1024"),
    ("(guint) META_DEBUG_KMS", "2097152"),
    ("(guint) META_DEBUG_PAINT_NONE", "0"),
    ("(guint) META_DEBUG_PAINT_OPAQUE_REGION", "1"),
    ("(guint) META_DEBUG_PING", "512"),
    ("(guint) META_DEBUG_PLACEMENT", "256"),
    ("(guint) META_DEBUG_PREFS", "8192"),
    ("(guint) META_DEBUG_REMOTE_DESKTOP", "8388608"),
    ("(guint) META_DEBUG_RESIZING", "32768"),
    ("(guint) META_DEBUG_SCREEN_CAST", "4194304"),
    ("(guint) META_DEBUG_SHAPES", "65536"),
    ("(guint) META_DEBUG_SM", "8"),
    ("(guint) META_DEBUG_STACK", "4"),
    ("(guint) META_DEBUG_STARTUP", "4096"),
    ("(guint) META_DEBUG_SYNC", "2048"),
    ("(guint) META_DEBUG_VERBOSE", "-1"),
    ("(guint) META_DEBUG_WAYLAND", "1048576"),
    ("(guint) META_DEBUG_WINDOW_OPS", "64"),
    ("(guint) META_DEBUG_WINDOW_STATE", "32"),
    ("(guint) META_DEBUG_WORKAREA", "2"),
    ("META_DEFAULT_ICON_NAME", "window"),
    ("(guint) META_DIRECTION_BOTTOM", "8"),
    ("(guint) META_DIRECTION_DOWN", "8"),
    ("(guint) META_DIRECTION_HORIZONTAL", "3"),
    ("(guint) META_DIRECTION_LEFT", "1"),
    ("(guint) META_DIRECTION_RIGHT", "2"),
    ("(guint) META_DIRECTION_TOP", "4"),
    ("(guint) META_DIRECTION_UP", "4"),
    ("(guint) META_DIRECTION_VERTICAL", "12"),
    ("(gint) META_DISPLAY_BOTTOMLEFT", "2"),
    ("(gint) META_DISPLAY_BOTTOMRIGHT", "3"),
    ("(gint) META_DISPLAY_DOWN", "1"),
    ("(gint) META_DISPLAY_LEFT", "2"),
    ("(gint) META_DISPLAY_RIGHT", "3"),
    ("(gint) META_DISPLAY_TOPLEFT", "0"),
    ("(gint) META_DISPLAY_TOPRIGHT", "1"),
    ("(gint) META_DISPLAY_UP", "0"),
    ("(gint) META_EDGE_MONITOR", "1"),
    ("(gint) META_EDGE_SCREEN", "2"),
    ("(gint) META_EDGE_WINDOW", "0"),
    ("(gint) META_EXIT_ERROR", "1"),
    ("(gint) META_EXIT_SUCCESS", "0"),
    ("(guint) META_FRAME_ABOVE", "8192"),
    ("(guint) META_FRAME_ALLOWS_DELETE", "1"),
    ("(guint) META_FRAME_ALLOWS_HORIZONTAL_RESIZE", "32"),
    ("(guint) META_FRAME_ALLOWS_MAXIMIZE", "8"),
    ("(guint) META_FRAME_ALLOWS_MENU", "2"),
    ("(guint) META_FRAME_ALLOWS_MINIMIZE", "4"),
    ("(guint) META_FRAME_ALLOWS_MOVE", "2048"),
    ("(guint) META_FRAME_ALLOWS_SHADE", "1024"),
    ("(guint) META_FRAME_ALLOWS_VERTICAL_RESIZE", "16"),
    ("(guint) META_FRAME_FULLSCREEN", "4096"),
    ("(guint) META_FRAME_HAS_FOCUS", "64"),
    ("(guint) META_FRAME_MAXIMIZED", "512"),
    ("(guint) META_FRAME_SHADED", "128"),
    ("(guint) META_FRAME_STUCK", "256"),
    ("(guint) META_FRAME_TILED_LEFT", "16384"),
    ("(guint) META_FRAME_TILED_RIGHT", "32768"),
    ("(gint) META_FRAME_TYPE_ATTACHED", "6"),
    ("(gint) META_FRAME_TYPE_BORDER", "5"),
    ("(gint) META_FRAME_TYPE_DIALOG", "1"),
    ("(gint) META_FRAME_TYPE_LAST", "7"),
    ("(gint) META_FRAME_TYPE_MENU", "4"),
    ("(gint) META_FRAME_TYPE_MODAL_DIALOG", "2"),
    ("(gint) META_FRAME_TYPE_NORMAL", "0"),
    ("(gint) META_FRAME_TYPE_UTILITY", "3"),
    ("(gint) META_GRAB_OP_COMPOSITOR", "2"),
    ("(gint) META_GRAB_OP_FRAME_BUTTON", "4"),
    ("(gint) META_GRAB_OP_KEYBOARD_MOVING", "257"),
    ("(gint) META_GRAB_OP_KEYBOARD_RESIZING_E", "8449"),
    ("(gint) META_GRAB_OP_KEYBOARD_RESIZING_N", "33025"),
    ("(gint) META_GRAB_OP_KEYBOARD_RESIZING_NE", "41217"),
    ("(gint) META_GRAB_OP_KEYBOARD_RESIZING_NW", "37121"),
    ("(gint) META_GRAB_OP_KEYBOARD_RESIZING_S", "16641"),
    ("(gint) META_GRAB_OP_KEYBOARD_RESIZING_SE", "24833"),
    ("(gint) META_GRAB_OP_KEYBOARD_RESIZING_SW", "20737"),
    ("(gint) META_GRAB_OP_KEYBOARD_RESIZING_UNKNOWN", "769"),
    ("(gint) META_GRAB_OP_KEYBOARD_RESIZING_W", "4353"),
    ("(gint) META_GRAB_OP_MOVING", "1"),
    ("(gint) META_GRAB_OP_NONE", "0"),
    ("(gint) META_GRAB_OP_RESIZING_E", "8193"),
    ("(gint) META_GRAB_OP_RESIZING_N", "32769"),
    ("(gint) META_GRAB_OP_RESIZING_NE", "40961"),
    ("(gint) META_GRAB_OP_RESIZING_NW", "36865"),
    ("(gint) META_GRAB_OP_RESIZING_S", "16385"),
    ("(gint) META_GRAB_OP_RESIZING_SE", "24577"),
    ("(gint) META_GRAB_OP_RESIZING_SW", "20481"),
    ("(gint) META_GRAB_OP_RESIZING_W", "4097"),
    ("(gint) META_GRAB_OP_WAYLAND_POPUP", "3"),
    ("(gint) META_GRAB_OP_WINDOW_BASE", "1"),
    ("(gint) META_GRAVITY_CENTER", "5"),
    ("(gint) META_GRAVITY_EAST", "6"),
    ("(gint) META_GRAVITY_NONE", "0"),
    ("(gint) META_GRAVITY_NORTH", "2"),
    ("(gint) META_GRAVITY_NORTH_EAST", "3"),
    ("(gint) META_GRAVITY_NORTH_WEST", "1"),
    ("(gint) META_GRAVITY_SOUTH", "8"),
    ("(gint) META_GRAVITY_SOUTH_EAST", "9"),
    ("(gint) META_GRAVITY_SOUTH_WEST", "7"),
    ("(gint) META_GRAVITY_STATIC", "10"),
    ("(gint) META_GRAVITY_WEST", "4"),
    ("META_ICON_HEIGHT", "96"),
    ("META_ICON_WIDTH", "96"),
    ("(gint) META_INHIBIT_SHORTCUTS_DIALOG_RESPONSE_ALLOW", "0"),
    ("(gint) META_INHIBIT_SHORTCUTS_DIALOG_RESPONSE_DENY", "1"),
    ("(gint) META_KEYBINDING_ACTION_ACTIVATE_WINDOW_MENU", "37"),
    ("(gint) META_KEYBINDING_ACTION_ALWAYS_ON_TOP", "89"),
    ("(gint) META_KEYBINDING_ACTION_BEGIN_MOVE", "48"),
    ("(gint) META_KEYBINDING_ACTION_BEGIN_RESIZE", "49"),
    ("(gint) META_KEYBINDING_ACTION_CLOSE", "47"),
    ("(gint) META_KEYBINDING_ACTION_CYCLE_GROUP", "26"),
    ("(gint) META_KEYBINDING_ACTION_CYCLE_GROUP_BACKWARD", "27"),
    ("(gint) META_KEYBINDING_ACTION_CYCLE_PANELS", "30"),
    ("(gint) META_KEYBINDING_ACTION_CYCLE_PANELS_BACKWARD", "31"),
    ("(gint) META_KEYBINDING_ACTION_CYCLE_WINDOWS", "28"),
    ("(gint) META_KEYBINDING_ACTION_CYCLE_WINDOWS_BACKWARD", "29"),
    ("(gint) META_KEYBINDING_ACTION_ISO_NEXT_GROUP", "88"),
    ("(gint) META_KEYBINDING_ACTION_LAST", "92"),
    ("(gint) META_KEYBINDING_ACTION_LOCATE_POINTER_KEY", "87"),
    ("(gint) META_KEYBINDING_ACTION_LOWER", "74"),
    ("(gint) META_KEYBINDING_ACTION_MAXIMIZE", "43"),
    ("(gint) META_KEYBINDING_ACTION_MAXIMIZE_HORIZONTALLY", "76"),
    ("(gint) META_KEYBINDING_ACTION_MAXIMIZE_VERTICALLY", "75"),
    ("(gint) META_KEYBINDING_ACTION_MINIMIZE", "46"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_CENTER", "85"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_CORNER_NE", "78"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_CORNER_NW", "77"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_CORNER_SE", "80"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_CORNER_SW", "79"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_MONITOR_DOWN", "71"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_MONITOR_LEFT", "68"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_MONITOR_RIGHT", "69"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_MONITOR_UP", "70"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_SIDE_E", "83"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_SIDE_N", "81"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_SIDE_S", "82"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_SIDE_W", "84"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_1", "51"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_10", "60"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_11", "61"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_12", "62"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_2", "52"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_3", "53"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_4", "54"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_5", "55"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_6", "56"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_7", "57"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_8", "58"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_9", "59"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_DOWN", "66"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_LAST", "67"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_LEFT", "63"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_RIGHT", "64"),
    ("(gint) META_KEYBINDING_ACTION_MOVE_TO_WORKSPACE_UP", "65"),
    ("(gint) META_KEYBINDING_ACTION_NONE", "0"),
    ("(gint) META_KEYBINDING_ACTION_OVERLAY_KEY", "86"),
    ("(gint) META_KEYBINDING_ACTION_PANEL_MAIN_MENU", "33"),
    ("(gint) META_KEYBINDING_ACTION_PANEL_RUN_DIALOG", "34"),
    ("(gint) META_KEYBINDING_ACTION_RAISE", "73"),
    ("(gint) META_KEYBINDING_ACTION_RAISE_OR_LOWER", "72"),
    ("(gint) META_KEYBINDING_ACTION_ROTATE_MONITOR", "91"),
    ("(gint) META_KEYBINDING_ACTION_SET_SPEW_MARK", "36"),
    ("(gint) META_KEYBINDING_ACTION_SHOW_DESKTOP", "32"),
    ("(gint) META_KEYBINDING_ACTION_SWITCH_APPLICATIONS", "18"),
    ("(gint) META_KEYBINDING_ACTION_SWITCH_APPLICATIONS_BACKWARD", "19"),
    ("(gint) META_KEYBINDING_ACTION_SWITCH_GROUP", "20"),
    ("(gint) META_KEYBINDING_ACTION_SWITCH_GROUP_BACKWARD", "21"),
    ("(gint) META_KEYBINDING_ACTION_SWITCH_MONITOR", "90"),
    ("(gint) META_KEYBINDING_ACTION_SWITCH_PANELS", "24"),
    ("(gint) META_KEYBINDING_ACTION_SWITCH_PANELS_BACKWARD", "25"),
    ("(gint) META_KEYBINDING_ACTION_SWITCH_WINDOWS", "22"),
    ("(gint) META_KEYBINDING_ACTION_SWITCH_WINDOWS_BACKWARD", "23"),
    ("(gint) META_KEYBINDING_ACTION_TOGGLE_ABOVE", "42"),
    ("(gint) META_KEYBINDING_ACTION_TOGGLE_FULLSCREEN", "38"),
    ("(gint) META_KEYBINDING_ACTION_TOGGLE_MAXIMIZED", "39"),
    ("(gint) META_KEYBINDING_ACTION_TOGGLE_ON_ALL_WORKSPACES", "50"),
    ("(gint) META_KEYBINDING_ACTION_TOGGLE_RECORDING", "35"),
    ("(gint) META_KEYBINDING_ACTION_TOGGLE_SHADED", "45"),
    ("(gint) META_KEYBINDING_ACTION_TOGGLE_TILED_LEFT", "40"),
    ("(gint) META_KEYBINDING_ACTION_TOGGLE_TILED_RIGHT", "41"),
    ("(gint) META_KEYBINDING_ACTION_UNMAXIMIZE", "44"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_1", "1"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_10", "10"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_11", "11"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_12", "12"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_2", "2"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_3", "3"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_4", "4"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_5", "5"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_6", "6"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_7", "7"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_8", "8"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_9", "9"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_DOWN", "16"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_LAST", "17"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_LEFT", "13"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_RIGHT", "14"),
    ("(gint) META_KEYBINDING_ACTION_WORKSPACE_UP", "15"),
    ("(guint) META_KEY_BINDING_BUILTIN", "2"),
    ("(guint) META_KEY_BINDING_IGNORE_AUTOREPEAT", "16"),
    ("(guint) META_KEY_BINDING_IS_REVERSED", "4"),
    ("(guint) META_KEY_BINDING_NONE", "0"),
    ("(guint) META_KEY_BINDING_NON_MASKABLE", "8"),
    ("(guint) META_KEY_BINDING_NO_AUTO_GRAB", "32"),
    ("(guint) META_KEY_BINDING_PER_WINDOW", "1"),
    ("(gint) META_LATER_BEFORE_REDRAW", "4"),
    ("(gint) META_LATER_CALC_SHOWING", "1"),
    ("(gint) META_LATER_CHECK_FULLSCREEN", "2"),
    ("(gint) META_LATER_IDLE", "5"),
    ("(gint) META_LATER_RESIZE", "0"),
    ("(gint) META_LATER_SYNC_STACK", "3"),
    ("(gint) META_LAYER_BOTTOM", "1"),
    ("(gint) META_LAYER_DESKTOP", "0"),
    ("(gint) META_LAYER_DOCK", "4"),
    ("(gint) META_LAYER_LAST", "8"),
    ("(gint) META_LAYER_NORMAL", "2"),
    ("(gint) META_LAYER_OVERRIDE_REDIRECT", "7"),
    ("(gint) META_LAYER_TOP", "4"),
    ("(gint) META_LOCALE_DIRECTION_LTR", "0"),
    ("(gint) META_LOCALE_DIRECTION_RTL", "1"),
    ("(guint) META_MAXIMIZE_BOTH", "3"),
    ("(guint) META_MAXIMIZE_HORIZONTAL", "1"),
    ("(guint) META_MAXIMIZE_VERTICAL", "2"),
    ("META_MINI_ICON_HEIGHT", "16"),
    ("META_MINI_ICON_WIDTH", "16"),
    ("(guint) META_MODAL_KEYBOARD_ALREADY_GRABBED", "2"),
    ("(guint) META_MODAL_POINTER_ALREADY_GRABBED", "1"),
    ("(gint) META_MONITOR_SWITCH_CONFIG_ALL_LINEAR", "1"),
    ("(gint) META_MONITOR_SWITCH_CONFIG_ALL_MIRROR", "0"),
    ("(gint) META_MONITOR_SWITCH_CONFIG_BUILTIN", "3"),
    ("(gint) META_MONITOR_SWITCH_CONFIG_EXTERNAL", "2"),
    ("(gint) META_MONITOR_SWITCH_CONFIG_UNKNOWN", "4"),
    ("(gint) META_MOTION_DOWN", "-2"),
    ("(gint) META_MOTION_DOWN_LEFT", "-7"),
    ("(gint) META_MOTION_DOWN_RIGHT", "-8"),
    ("(gint) META_MOTION_LEFT", "-3"),
    ("(gint) META_MOTION_RIGHT", "-4"),
    ("(gint) META_MOTION_UP", "-1"),
    ("(gint) META_MOTION_UP_LEFT", "-5"),
    ("(gint) META_MOTION_UP_RIGHT", "-6"),
    ("(gint) META_N_SELECTION_TYPES", "3"),
    ("(gint) META_PAD_ACTION_BUTTON", "0"),
    ("(gint) META_PAD_ACTION_RING", "1"),
    ("(gint) META_PAD_ACTION_STRIP", "2"),
    ("(gint) META_PREF_ACTION_DOUBLE_CLICK_TITLEBAR", "5"),
    ("(gint) META_PREF_ACTION_MIDDLE_CLICK_TITLEBAR", "6"),
    ("(gint) META_PREF_ACTION_RIGHT_CLICK_TITLEBAR", "7"),
    ("(gint) META_PREF_ATTACH_MODAL_DIALOGS", "3"),
    ("(gint) META_PREF_AUDIBLE_BELL", "19"),
    ("(gint) META_PREF_AUTO_MAXIMIZE", "30"),
    ("(gint) META_PREF_AUTO_RAISE", "8"),
    ("(gint) META_PREF_AUTO_RAISE_DELAY", "9"),
    ("(gint) META_PREF_BUTTON_LAYOUT", "16"),
    ("(gint) META_PREF_CENTER_NEW_WINDOWS", "31"),
    ("(gint) META_PREF_CHECK_ALIVE_TIMEOUT", "34"),
    ("(gint) META_PREF_CURSOR_SIZE", "24"),
    ("(gint) META_PREF_CURSOR_THEME", "23"),
    ("(gint) META_PREF_DISABLE_WORKAROUNDS", "15"),
    ("(gint) META_PREF_DRAGGABLE_BORDER_WIDTH", "29"),
    ("(gint) META_PREF_DRAG_THRESHOLD", "32"),
    ("(gint) META_PREF_DYNAMIC_WORKSPACES", "13"),
    ("(gint) META_PREF_EDGE_TILING", "26"),
    ("(gint) META_PREF_FOCUS_CHANGE_ON_POINTER_REST", "10"),
    ("(gint) META_PREF_FOCUS_MODE", "1"),
    ("(gint) META_PREF_FOCUS_NEW_WINDOWS", "2"),
    ("(gint) META_PREF_FORCE_FULLSCREEN", "27"),
    ("(gint) META_PREF_GNOME_ACCESSIBILITY", "21"),
    ("(gint) META_PREF_GNOME_ANIMATIONS", "22"),
    ("(gint) META_PREF_KEYBINDINGS", "14"),
    ("(gint) META_PREF_LOCATE_POINTER", "33"),
    ("(gint) META_PREF_MOUSE_BUTTON_MODS", "0"),
    ("(gint) META_PREF_NUM_WORKSPACES", "12"),
    ("(gint) META_PREF_RAISE_ON_CLICK", "4"),
    ("(gint) META_PREF_RESIZE_WITH_RIGHT_BUTTON", "25"),
    ("(gint) META_PREF_TITLEBAR_FONT", "11"),
    ("(gint) META_PREF_VISUAL_BELL", "18"),
    ("(gint) META_PREF_VISUAL_BELL_TYPE", "20"),
    ("(gint) META_PREF_WORKSPACES_ONLY_ON_PRIMARY", "28"),
    ("(gint) META_PREF_WORKSPACE_NAMES", "17"),
    ("META_PRIORITY_BEFORE_REDRAW", "40"),
    ("META_PRIORITY_PREFS_NOTIFY", "10"),
    ("META_PRIORITY_REDRAW", "50"),
    ("META_PRIORITY_RESIZE", "15"),
    ("(gint) META_SELECTION_CLIPBOARD", "1"),
    ("(gint) META_SELECTION_DND", "2"),
    ("(gint) META_SELECTION_PRIMARY", "0"),
    ("(gint) META_SHADOW_MODE_AUTO", "0"),
    ("(gint) META_SHADOW_MODE_FORCED_OFF", "1"),
    ("(gint) META_SHADOW_MODE_FORCED_ON", "2"),
    ("(gint) META_SIDE_BOTTOM", "8"),
    ("(gint) META_SIDE_LEFT", "1"),
    ("(gint) META_SIDE_RIGHT", "2"),
    ("(gint) META_SIDE_TOP", "4"),
    ("(gint) META_SIZE_CHANGE_FULLSCREEN", "2"),
    ("(gint) META_SIZE_CHANGE_MAXIMIZE", "0"),
    ("(gint) META_SIZE_CHANGE_UNFULLSCREEN", "3"),
    ("(gint) META_SIZE_CHANGE_UNMAXIMIZE", "1"),
    ("(gint) META_TAB_LIST_DOCKS", "1"),
    ("(gint) META_TAB_LIST_GROUP", "2"),
    ("(gint) META_TAB_LIST_NORMAL", "0"),
    ("(gint) META_TAB_LIST_NORMAL_ALL", "3"),
    ("(gint) META_TAB_SHOW_ICON", "0"),
    ("(gint) META_TAB_SHOW_INSTANTLY", "1"),
    ("(guint) META_VIRTUAL_ALT_MASK", "128"),
    ("(guint) META_VIRTUAL_CONTROL_MASK", "64"),
    ("META_VIRTUAL_CORE_KEYBOARD_ID", "3"),
    ("META_VIRTUAL_CORE_POINTER_ID", "2"),
    ("(guint) META_VIRTUAL_HYPER_MASK", "1024"),
    ("(guint) META_VIRTUAL_META_MASK", "256"),
    ("(guint) META_VIRTUAL_MOD2_MASK", "2048"),
    ("(guint) META_VIRTUAL_MOD3_MASK", "4096"),
    ("(guint) META_VIRTUAL_MOD4_MASK", "8192"),
    ("(guint) META_VIRTUAL_MOD5_MASK", "16384"),
    ("(guint) META_VIRTUAL_SHIFT_MASK", "32"),
    ("(guint) META_VIRTUAL_SUPER_MASK", "512"),
    ("(gint) META_WINDOW_CLIENT_TYPE_WAYLAND", "0"),
    ("(gint) META_WINDOW_CLIENT_TYPE_X11", "1"),
    ("(gint) META_WINDOW_COMBO", "13"),
    ("(gint) META_WINDOW_DESKTOP", "1"),
    ("(gint) META_WINDOW_DIALOG", "3"),
    ("(gint) META_WINDOW_DND", "14"),
    ("(gint) META_WINDOW_DOCK", "2"),
    ("(gint) META_WINDOW_DROPDOWN_MENU", "9"),
    ("(gint) META_WINDOW_MENU", "6"),
    ("(gint) META_WINDOW_MENU_APP", "1"),
    ("(gint) META_WINDOW_MENU_WM", "0"),
    ("(gint) META_WINDOW_MODAL_DIALOG", "4"),
    ("(gint) META_WINDOW_NORMAL", "0"),
    ("(gint) META_WINDOW_NOTIFICATION", "12"),
    ("(gint) META_WINDOW_OVERRIDE_OTHER", "15"),
    ("(gint) META_WINDOW_POPUP_MENU", "10"),
    ("(gint) META_WINDOW_SPLASHSCREEN", "8"),
    ("(gint) META_WINDOW_TOOLBAR", "5"),
    ("(gint) META_WINDOW_TOOLTIP", "11"),
    ("(gint) META_WINDOW_UTILITY", "7"),
];


