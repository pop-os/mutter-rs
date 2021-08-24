<!-- file * -->
<!-- struct Background -->
This class handles tracking and painting the root window background.
By integrating with `MetaWindowGroup` we can avoid painting parts of
the background that are obscured by other windows.

# Implements

[`trait@glib::ObjectExt`]
<!-- struct BackgroundActor -->
This class handles tracking and painting the root window background.
By integrating with `MetaWindowGroup` we can avoid painting parts of
the background that are obscured by other windows.

# Implements

[`trait@clutter::prelude::ActorExt`], [`trait@glib::ObjectExt`]
<!-- struct BackgroundContent -->
This class handles tracking and painting the root window background.
By integrating with `MetaWindowGroup` we can avoid painting parts of
the background that are obscured by other windows.

# Implements

[`trait@glib::ObjectExt`], [`trait@clutter::prelude::ContentExt`]
<!-- impl BackgroundContent::fn set_rounded_clip_bounds -->
Sets the bounding clip rectangle of the [`BackgroundContent`][crate::BackgroundContent] that's used
when a rounded clip set via [`set_rounded_clip_radius()`][Self::set_rounded_clip_radius()]
is in effect, set it to [`None`] to use no bounding clip, rounding the edges
of the full texture.
## `bounds`
The new bounding clip rectangle, or [`None`]
<!-- struct BackgroundGroup -->


# Implements

[`trait@clutter::prelude::ActorExt`], [`trait@glib::ObjectExt`]
<!-- enum Cursor::variant Default -->
Default cursor
<!-- enum Cursor::variant NorthResize -->
Resize northern edge cursor
<!-- enum Cursor::variant SouthResize -->
Resize southern edge cursor
<!-- enum Cursor::variant WestResize -->
Resize western edge cursor
<!-- enum Cursor::variant EastResize -->
Resize eastern edge cursor
<!-- enum Cursor::variant SeResize -->
Resize south-eastern corner cursor
<!-- enum Cursor::variant SwResize -->
Resize south-western corner cursor
<!-- enum Cursor::variant NeResize -->
Resize north-eastern corner cursor
<!-- enum Cursor::variant NwResize -->
Resize north-western corner cursor
<!-- enum Cursor::variant MoveOrResizeWindow -->
Move or resize cursor
<!-- enum Cursor::variant Busy -->
Busy cursor
<!-- enum Cursor::variant DndInDrag -->
DND in drag cursor
<!-- enum Cursor::variant DndMove -->
DND move cursor
<!-- enum Cursor::variant DndCopy -->
DND copy cursor
<!-- enum Cursor::variant DndUnsupportedTarget -->
DND unsupported target
<!-- enum Cursor::variant PointingHand -->
pointing hand
<!-- enum Cursor::variant Crosshair -->
crosshair (action forbidden)
<!-- enum Cursor::variant Ibeam -->
I-beam (text input)
<!-- enum Cursor::variant Blank -->
Invisible cursor
<!-- struct Display -->


# Implements

[`trait@glib::ObjectExt`]
<!-- impl Display::fn add_keybinding -->
Add a keybinding at runtime. The key `name` in `schema` needs to be of
type `G_VARIANT_TYPE_STRING_ARRAY`, with each string describing a
keybinding in the form of "&lt;Control&gt;a" or "&lt;Shift&gt;&lt;Alt&gt;F1". The parser
is fairly liberal and allows lower or upper case, and also abbreviations
such as "&lt;Ctl&gt;" and "&lt;Ctrl&gt;". If the key is set to the empty list or a
list with a single element of either "" or "disabled", the keybinding is
disabled.

Use [`remove_keybinding()`][Self::remove_keybinding()] to remove the binding.
## `name`
the binding's name
## `settings`
the [`gio::Settings`][crate::gio::Settings] object where `name` is stored
## `flags`
flags to specify binding details
## `handler`
function to run when the keybinding is invoked
## `free_data`
function to free `user_data`

# Returns

the corresponding keybinding action if the keybinding was
 added successfully, otherwise `META_KEYBINDING_ACTION_NONE`
<!-- impl Display::fn sound_player -->

# Returns

The sound player of the display
<!-- impl Display::fn is_pointer_emulating_sequence -->
Tells whether the event sequence is the used for pointer emulation
and single-touch interaction.
## `sequence`
a `ClutterEventSequence`

# Returns

[`true`] if the sequence emulates pointer behavior
<!-- impl Display::fn sort_windows_by_stacking -->
Sorts a set of windows according to their current stacking order. If windows
from multiple screens are present in the set of input windows, then all the
windows on screen 0 are sorted below all the windows on screen 1, and so forth.
Since the stacking order of override-redirect windows isn't controlled by
Metacity, if override-redirect windows are in the input, the result may not
correspond to the actual stacking order in the X server.

An example of using this would be to sort the list of transient dialogs for a
window into their current stacking order.
## `windows`
Set of windows

# Returns

Input windows sorted by stacking order, from lowest to highest
<!-- enum DisplayCorner::variant Topleft -->
top-left corner
<!-- enum DisplayCorner::variant Topright -->
top-right corner
<!-- enum DisplayCorner::variant Bottomleft -->
bottom-left corner
<!-- enum DisplayCorner::variant Bottomright -->
bottom-right corner
<!-- enum DisplayDirection::variant Up -->
up
<!-- enum DisplayDirection::variant Down -->
down
<!-- enum DisplayDirection::variant Left -->
left
<!-- enum DisplayDirection::variant Right -->
right
<!-- enum FrameType::variant Normal -->
Normal frame
<!-- enum FrameType::variant Dialog -->
Dialog frame
<!-- enum FrameType::variant ModalDialog -->
Modal dialog frame
<!-- enum FrameType::variant Utility -->
Utility frame
<!-- enum FrameType::variant Menu -->
Menu frame
<!-- enum FrameType::variant Border -->
Border frame
<!-- enum FrameType::variant Attached -->
Attached frame
<!-- enum FrameType::variant Last -->
Marks the end of the [`FrameType`][crate::FrameType] enumeration
<!-- enum GrabOp::variant None -->
None
<!-- enum GrabOp::variant Compositor -->
Compositor asked for grab
<!-- enum GrabOp::variant Moving -->
Moving with pointer
<!-- enum GrabOp::variant ResizingNw -->
Resizing NW with pointer
<!-- enum GrabOp::variant ResizingN -->
Resizing N with pointer
<!-- enum GrabOp::variant ResizingNe -->
Resizing NE with pointer
<!-- enum GrabOp::variant ResizingE -->
Resizing E with pointer
<!-- enum GrabOp::variant ResizingSw -->
Resizing SW with pointer
<!-- enum GrabOp::variant ResizingS -->
Resizing S with pointer
<!-- enum GrabOp::variant ResizingSe -->
Resizing SE with pointer
<!-- enum GrabOp::variant ResizingW -->
Resizing W with pointer
<!-- enum GrabOp::variant KeyboardMoving -->
Moving with keyboard
<!-- enum GrabOp::variant KeyboardResizingUnknown -->
Resizing with keyboard
<!-- enum GrabOp::variant KeyboardResizingNw -->
Resizing NS with keyboard
<!-- enum GrabOp::variant KeyboardResizingN -->
Resizing N with keyboard
<!-- enum GrabOp::variant KeyboardResizingNe -->
Resizing NE with keyboard
<!-- enum GrabOp::variant KeyboardResizingE -->
Resizing E with keyboard
<!-- enum GrabOp::variant KeyboardResizingSw -->
Resizing SW with keyboard
<!-- enum GrabOp::variant KeyboardResizingS -->
Resizing S with keyboard
<!-- enum GrabOp::variant KeyboardResizingSe -->
Resizing SE with keyboard
<!-- enum GrabOp::variant KeyboardResizingW -->
Resizing W with keyboard
<!-- struct KeyBindingFlags::const NONE -->
none
<!-- struct KeyBindingFlags::const PER_WINDOW -->
per-window
<!-- struct KeyBindingFlags::const BUILTIN -->
built-in
<!-- struct KeyBindingFlags::const IS_REVERSED -->
is reversed
<!-- struct KeyBindingFlags::const NON_MASKABLE -->
always active
<!-- struct KeyBindingFlags::const NO_AUTO_GRAB -->
not grabbed automatically
<!-- struct MaximizeFlags::const HORIZONTAL -->
Horizontal
<!-- struct MaximizeFlags::const VERTICAL -->
Vertical
<!-- struct MaximizeFlags::const BOTH -->
Both
<!-- struct ModalOptions -->
Options that can be provided when calling [`PluginExt::begin_modal()`][crate::prelude::PluginExt::begin_modal()].
<!-- struct ModalOptions::const POINTER_ALREADY_GRABBED -->
if set the pointer is already
 grabbed by the plugin and should not be grabbed again.
<!-- struct ModalOptions::const KEYBOARD_ALREADY_GRABBED -->
if set the keyboard is already
 grabbed by the plugin and should not be grabbed again.
<!-- enum MotionDirection::variant Up -->
Upwards motion
<!-- enum MotionDirection::variant Down -->
Downwards motion
<!-- enum MotionDirection::variant Left -->
Motion to the left
<!-- enum MotionDirection::variant Right -->
Motion to the right
<!-- enum MotionDirection::variant UpLeft -->
Motion up and to the left
<!-- enum MotionDirection::variant UpRight -->
Motion up and to the right
<!-- enum MotionDirection::variant DownLeft -->
Motion down and to the left
<!-- enum MotionDirection::variant DownRight -->
Motion down and to the right
<!-- struct Plugin -->


This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`PluginExt`][trait@crate::prelude::PluginExt], [`trait@glib::ObjectExt`]
<!-- struct Selection -->


# Implements

[`trait@glib::ObjectExt`]
<!-- impl Selection::fn transfer_finish -->
Finishes the transfer of a queried mimetype.
## `result`
The async result

# Returns

[`true`] if the transfer was successful.
<!-- struct SelectionSource -->


# Implements

[`SelectionSourceExt`][trait@crate::prelude::SelectionSourceExt], [`trait@glib::ObjectExt`]
<!-- trait SelectionSourceExt::fn read_finish -->
Finishes a read from the selection source.
## `result`
The async result

# Returns

The resulting [`gio::InputStream`][crate::gio::InputStream]
<!-- enum StackLayer::variant Desktop -->
Desktop layer
<!-- enum StackLayer::variant Bottom -->
Bottom layer
<!-- enum StackLayer::variant Normal -->
Normal layer
<!-- enum StackLayer::variant Top -->
Top layer
<!-- enum StackLayer::variant Dock -->
Dock layer
<!-- enum StackLayer::variant OverrideRedirect -->
Override-redirect layer
<!-- enum StackLayer::variant Last -->
Marks the end of the [`StackLayer`][crate::StackLayer] enumeration
<!-- enum TabList::variant Normal -->
Normal windows
<!-- enum TabList::variant Docks -->
Dock windows
<!-- enum TabList::variant Group -->
Groups
<!-- enum TabList::variant NormalAll -->
All windows
<!-- struct Window -->


This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`trait@glib::ObjectExt`]
<!-- impl Window::fn set_icon_geometry -->
Sets or unsets the location of the icon corresponding to the window. If
set, the location should correspond to a dock, task bar or other user
interface element displaying the icon, and is relative to the root window.
## `rect`
rectangle with the desired geometry or [`None`].
<!-- struct WindowActor -->


This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`trait@clutter::prelude::ActorExt`], [`trait@glib::ObjectExt`]
<!-- impl WindowActor::fn get_image -->
Flattens the layers of `self` into one ARGB32 image by alpha blending
the images, and returns the flattened image.
## `clip`
A clipping rectangle, to help prevent extra processing.
In the case that the clipping rectangle is partially or fully
outside the bounds of the actor, the rectangle will be clipped.

# Returns

a new cairo surface to be freed with
`cairo_surface_destroy()`.
<!-- impl WindowActor::fn texture -->
Gets the ClutterActor that is used to display the contents of the window,
or NULL if no texture is shown yet, because the window is not mapped.

# Returns

the [`clutter::Actor`][crate::clutter::Actor] for the contents
<!-- enum WindowClientType::variant Wayland -->
A Wayland based window
<!-- enum WindowClientType::variant X11 -->
An X11 based window
<!-- enum WindowType::variant Normal -->
Normal
<!-- enum WindowType::variant Desktop -->
Desktop
<!-- enum WindowType::variant Dock -->
Dock
<!-- enum WindowType::variant Dialog -->
Dialog
<!-- enum WindowType::variant ModalDialog -->
Modal dialog
<!-- enum WindowType::variant Toolbar -->
Toolbar
<!-- enum WindowType::variant Menu -->
Menu
<!-- enum WindowType::variant Utility -->
Utility
<!-- enum WindowType::variant Splashscreen -->
Splashcreen
<!-- enum WindowType::variant DropdownMenu -->
Dropdown menu
<!-- enum WindowType::variant PopupMenu -->
Popup menu
<!-- enum WindowType::variant Tooltip -->
Tooltip
<!-- enum WindowType::variant Notification -->
Notification
<!-- enum WindowType::variant Combo -->
Combobox
<!-- enum WindowType::variant Dnd -->
Drag and drop
<!-- enum WindowType::variant OverrideOther -->
Other override-redirect window type
<!-- struct Workspace -->


# Implements

[`trait@glib::ObjectExt`]
<!-- impl Workspace::fn set_builtin_struts -->
Sets a list of struts that will be used in addition to the struts
of the windows in the workspace when computing the work area of
the workspace.
## `struts`
list of `MetaStrut`
<!-- struct WorkspaceManager -->


# Implements

[`trait@glib::ObjectExt`]
