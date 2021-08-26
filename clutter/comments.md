<!-- file * -->
<!-- fn add_debug_flags -->
Adds the debug flags passed to the list of debug flags.
<!-- fn cairo_clear -->
Utility function to clear a Cairo context.
## `cr`
a Cairo context
<!-- fn cairo_set_source_color -->
Utility function for setting the source color of `cr` using
a [`Color`][crate::Color]. This function is the equivalent of:


```text
  cairo_set_source_rgba (cr,
                         color->red / 255.0,
                         color->green / 255.0,
                         color->blue / 255.0,
                         color->alpha / 255.0);
```
## `cr`
a Cairo context
## `color`
a [`Color`][crate::Color]
<!-- fn do_event -->
Processes an event.

The `event` must be a valid `ClutterEvent` and have a [`Stage`][crate::Stage]
associated to it.

This function is only useful when embedding Clutter inside another
toolkit, and it should never be called by applications.
## `event`
a `ClutterEvent`.
<!-- fn feature_available -->
Checks whether `feature` is available. `feature` can be a logical
OR of `ClutterFeatureFlags`.
## `feature`
a `ClutterFeatureFlags`

# Returns

[`true`] if a feature is available
<!-- fn feature_get_all -->
Returns all the supported features.

# Returns

a logical OR of all the supported features.
<!-- fn current_event -->
If an event is currently being processed, return that event.
This function is intended to be used to access event state
that might not be exposed by higher-level widgets. For
example, to get the key modifier state from a Button 'clicked'
event.

# Returns

The current ClutterEvent, or [`None`] if none
<!-- fn default_backend -->
Retrieves the default `ClutterBackend` used by Clutter. The
`ClutterBackend` holds backend-specific configuration options.

# Returns

the default backend. You should
 not ref or unref the returned object. Applications should rarely
 need to use this.
<!-- fn default_text_direction -->
Retrieves the default direction for the text. The text direction is
determined by the locale and/or by the `CLUTTER_TEXT_DIRECTION`
environment variable.

The default text direction can be overridden on a per-actor basis by using
`clutter_actor_set_text_direction()`.

# Returns

the default text direction
<!-- fn font_map -->
Retrieves the `PangoFontMap` instance used by Clutter.
You can use the global font map object with the COGL
Pango API.

# Returns

the `PangoFontMap` instance. The returned
 value is owned by Clutter and it should never be unreferenced.
<!-- fn option_group -->
Returns a `GOptionGroup` for the command line arguments recognized
by Clutter. You should add this group to your `GOptionContext` with
`g_option_context_add_group()`, if you are using `g_option_context_parse()`
to parse your commandline arguments.

Calling `g_option_context_parse()` with Clutter's `GOptionGroup` will result
in Clutter's initialization. That is, the following code:


```text
  g_option_context_set_main_group (context, clutter_get_option_group ());
  res = g_option_context_parse (context, &argc, &argc, NULL);
```

is functionally equivalent to:


```text
  clutter_init (&argc, &argv);
```

After `g_option_context_parse()` on a `GOptionContext` containing the
Clutter `GOptionGroup` has returned [`true`], Clutter is guaranteed to be
initialized.

# Returns

a `GOptionGroup` for the commandline arguments
 recognized by Clutter
<!-- fn option_group_without_init -->
Returns a `GOptionGroup` for the command line arguments recognized
by Clutter. You should add this group to your `GOptionContext` with
`g_option_context_add_group()`, if you are using `g_option_context_parse()`
to parse your commandline arguments.

Unlike `clutter_get_option_group()`, calling `g_option_context_parse()` with
the `GOptionGroup` returned by this function requires a subsequent explicit
call to `clutter_init()`; use this function when needing to set foreign
display connection with `clutter_x11_set_display()`, or with
``gtk_clutter_init()``.

# Returns

a `GOptionGroup` for the commandline arguments
 recognized by Clutter
<!-- fn script_id -->
Retrieves the Clutter script id, if any.
## `gobject`
a `GObject`

# Returns

the script id, or [`None`] if `object` was not defined inside
 a UI definition file. The returned string is owned by the object and
 should never be modified or freed.
<!-- fn init_with_args -->
This function does the same work as `clutter_init()`. Additionally,
it allows you to add your own command line options, and it
automatically generates nicely formatted `<option>`--help`</option>`
output. Note that your program will be terminated after writing
out the help output. Also note that, in case of error, the
error message will be placed inside `error` instead of being
printed on the display.

Just like `clutter_init()`, if this function returns an error code then
any subsequent call to any other Clutter API will result in undefined
behaviour - including segmentation faults.
## `argv`
a pointer to the array
 of command line arguments
## `parameter_string`
a string which is displayed in the
 first line of `<option>`--help`</option>` output, after
 `<literal>``<replaceable>`programname`</replaceable>` [OPTION...]`</literal>`
## `entries`
a [`None`] terminated array of
 `GOptionEntry`<!-- -->s describing the options of your program
## `translation_domain`
a translation domain to use for
 translating the `<option>`--help`</option>` output for the options in
 `entries` with `gettext()`, or [`None`]

# Returns

`CLUTTER_INIT_SUCCESS` if Clutter has been successfully
 initialised, or other values or `ClutterInitError` in case of
 error.
<!-- fn param_spec_color -->
Creates a `GParamSpec` for properties using [`Color`][crate::Color].
## `name`
name of the property
## `nick`
short name
## `blurb`
description (can be translatable)
## `default_value`
default value
## `flags`
flags for the param spec

# Returns

the newly created `GParamSpec`
<!-- fn param_spec_units -->
Creates a `GParamSpec` for properties using `ClutterUnits`.
## `name`
name of the property
## `nick`
short name
## `blurb`
description (can be translatable)
## `default_type`
the default type for the `ClutterUnits`
## `minimum`
lower boundary
## `maximum`
higher boundary
## `default_value`
default value
## `flags`
flags for the param spec

# Returns

the newly created `GParamSpec`
<!-- fn remove_debug_flags -->
Removes the debug flags passed from the list of debug flags.
<!-- fn threads_add_repaint_func_full -->
Adds a function to be called whenever Clutter is processing a new
frame.

If the function returns [`false`] it is automatically removed from the
list of repaint functions and will not be called again.

This function is guaranteed to be called from within the same thread
that called `clutter_main()`, and while the Clutter lock is being held;
the function will be called within the main loop, so it is imperative
that it does not block, otherwise the frame time budget may be lost.

A repaint function is useful to ensure that an update of the scenegraph
is performed before the scenegraph is repainted. The `flags` passed to this
function will determine the section of the frame processing that will
result in `func` being called.

Adding a repaint function does not automatically ensure that a new
frame will be queued.

When the repaint function is removed (either because it returned [`false`]
or because [`threads_remove_repaint_func()`][crate::threads_remove_repaint_func()] has been called) the
`notify` function will be called, if any is set.
## `flags`
flags for the repaint function
## `func`
the function to be called within the paint cycle
## `notify`
function to be called when removing the repaint
 function, or [`None`]

# Returns

the ID (greater than 0) of the repaint function. You
 can use the returned integer to remove the repaint function by
 calling [`threads_remove_repaint_func()`][crate::threads_remove_repaint_func()].
<!-- fn value_dup_paint_node -->
Retrieves a pointer to the `ClutterPaintNode` contained inside
the passed `GValue`, and if not [`None`] it will increase the
reference count.
## `value`
a `GValue` initialized with `CLUTTER_TYPE_PAINT_NODE`

# Returns

a pointer
 to the `ClutterPaintNode`, with its reference count increased,
 or [`None`]
<!-- fn value_get_color -->
Gets the [`Color`][crate::Color] contained in `value`.
## `value`
a `GValue` initialized to `CLUTTER_TYPE_COLOR`

# Returns

the color inside the passed `GValue`
<!-- fn value_get_paint_node -->
Retrieves a pointer to the `ClutterPaintNode` contained inside
the passed `GValue`.
## `value`
a `GValue` initialized with `CLUTTER_TYPE_PAINT_NODE`

# Returns

a pointer to
 a `ClutterPaintNode`, or [`None`]
<!-- fn value_get_shader_float -->
Retrieves the list of floating point values stored inside
the passed `GValue`. `value` must have been initialized with
`CLUTTER_TYPE_SHADER_FLOAT`.
## `value`
a `GValue`

# Returns

the pointer to a list of
 floating point values. The returned value is owned by the
 `GValue` and should never be modified or freed.
<!-- fn value_get_shader_int -->
Retrieves the list of integer values stored inside the passed
`GValue`. `value` must have been initialized with
`CLUTTER_TYPE_SHADER_INT`.
## `value`
a `GValue`

# Returns

the pointer to a list of
 integer values. The returned value is owned by the `GValue` and
 should never be modified or freed.
<!-- fn value_get_shader_matrix -->
Retrieves a matrix of floating point values stored inside
the passed `GValue`. `value` must have been initialized with
`CLUTTER_TYPE_SHADER_MATRIX`.
## `value`
a `GValue`

# Returns

the pointer to a matrix
 of floating point values. The returned value is owned by the `GValue` and
 should never be modified or freed.
<!-- fn value_get_units -->
Gets the `ClutterUnits` contained in `value`.
## `value`
a `GValue` initialized to `CLUTTER_TYPE_UNITS`

# Returns

the units inside the passed `GValue`
<!-- fn value_set_color -->
Sets `value` to `color`.
## `value`
a `GValue` initialized to `CLUTTER_TYPE_COLOR`
## `color`
the color to set
<!-- fn value_set_paint_node -->
Sets the contents of a `GValue` initialized with `CLUTTER_TYPE_PAINT_NODE`.

This function increased the reference count of `node`; if you do not wish
to increase the reference count, use `clutter_value_take_paint_node()`
instead. The reference count will be released by `g_value_unset()`.
## `value`
a `GValue` initialized with `CLUTTER_TYPE_PAINT_NODE`
## `node`
a `ClutterPaintNode`, or [`None`]
<!-- fn value_set_shader_float -->
Sets `floats` as the contents of `value`. The passed `GValue`
must have been initialized using `CLUTTER_TYPE_SHADER_FLOAT`.
## `value`
a `GValue`
## `floats`
an array of floating point values
<!-- fn value_set_shader_int -->
Sets `ints` as the contents of `value`. The passed `GValue`
must have been initialized using `CLUTTER_TYPE_SHADER_INT`.
## `value`
a `GValue`
## `ints`
an array of integer values
<!-- fn value_set_shader_matrix -->
Sets `matrix` as the contents of `value`. The passed `GValue`
must have been initialized using `CLUTTER_TYPE_SHADER_MATRIX`.
## `value`
a `GValue`
## `matrix`
a matrix of floating point values
<!-- fn value_set_units -->
Sets `value` to `units`
## `value`
a `GValue` initialized to `CLUTTER_TYPE_UNITS`
## `units`
the units to set
<!-- fn value_take_paint_node -->
Sets the contents of a `GValue` initialized with `CLUTTER_TYPE_PAINT_NODE`.

Unlike `clutter_value_set_paint_node()`, this function will not take a
reference on the passed `node`: instead, it will take ownership of the
current reference count.
## `value`
a `GValue`, initialized with `CLUTTER_TYPE_PAINT_NODE`
## `node`
a `ClutterPaintNode`, or [`None`]
<!-- const BUTTON_MIDDLE -->
The middle button of a pointer device.
<!-- const BUTTON_PRIMARY -->
The primary button of a pointer device.

This is typically the left mouse button in a right-handed
mouse configuration.
<!-- const BUTTON_SECONDARY -->
The secondary button of a pointer device.

This is typically the right mouse button in a right-handed
mouse configuration.
<!-- static COGL -->
Cogl (internal GL abstraction utility library) backend. Can be "gl" or
"gles" currently
<!-- const CURRENT_TIME -->
Default value for "now".
<!-- const EVENT_PROPAGATE -->
Continues the propagation of an event; this macro should be
used in event-related signals.
<!-- const EVENT_STOP -->
Stops the propagation of an event; this macro should be used
in event-related signals.
<!-- static FLAVOUR -->
GL Windowing system used
<!-- const NO_FPU -->
Set to 1 if Clutter was built without FPU (i.e fixed math), 0 otherwise
<!-- const PRIORITY_REDRAW -->
Priority of the redraws. This is chosen to be lower than the GTK+
redraw and resize priorities, because in application with both
GTK+ and Clutter it's more likely that the Clutter part will be
continually animating (and thus able to starve GTK+) than
vice-versa.
<!-- static STAGE_TYPE -->
The default GObject type for the Clutter stage.
<!-- struct Action -->


This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`ActorMetaExt`][trait@crate::prelude::ActorMetaExt]
<!-- struct Actor -->
Base class for actors.

# Implements

[`ActorExt`][trait@crate::prelude::ActorExt], [`AnimatableExt`][trait@crate::prelude::AnimatableExt], [`ContainerExt`][trait@crate::prelude::ContainerExt], [`ScriptableExt`][trait@crate::prelude::ScriptableExt]
<!-- trait ActorExt::fn add_effect -->
Adds `effect` to the list of `ClutterEffect`<!-- -->s applied to `self`

The [`Actor`][crate::Actor] will hold a reference on the `effect` until either
`clutter_actor_remove_effect()` or [`clear_effects()`][Self::clear_effects()] is
called.
## `effect`
a `ClutterEffect`
<!-- trait ActorExt::fn add_effect_with_name -->
A convenience function for setting the name of a `ClutterEffect`
while adding it to the list of effectss applied to `self`

This function is the logical equivalent of:



**⚠️ The following code is in C ⚠️**

```C
  clutter_actor_meta_set_name (CLUTTER_ACTOR_META (effect), name);
  clutter_actor_add_effect (self, effect);
```
## `name`
the name to set on the effect
## `effect`
a `ClutterEffect`
<!-- trait ActorExt::fn add_transition -->
Adds a `transition` to the [`Actor`][crate::Actor]'s list of animations.

The `name` string is a per-actor unique identifier of the `transition`: only
one `ClutterTransition` can be associated to the specified `name`.

The `transition` will be started once added.

This function will take a reference on the `transition`.

This function is usually called implicitly when modifying an animatable
property.
## `name`
the name of the transition to add
## `transition`
the `ClutterTransition` to add
<!-- trait ActorExt::fn allocate -->
Assigns the size of a [`Actor`][crate::Actor] from the given `box_`.

This function should only be called on the children of an actor when
overriding the `ClutterActorClass.allocate()` virtual function.

This function will adjust the stored allocation to take into account
the alignment flags set in the `property::Actor::x-align` and
`property::Actor::y-align` properties, as well as the margin values set in
the `property::Actor::margin-top`, `property::Actor::margin-right`,
`property::Actor::margin-bottom`, and `property::Actor::margin-left` properties.

This function will respect the easing state of the [`Actor`][crate::Actor] and
interpolate between the current allocation and the new one if the
easing state duration is a positive value.

Actors can know from their allocation box whether they have moved
with respect to their parent actor. The `flags` parameter describes
additional information about the allocation, for instance whether
the parent has moved with respect to the stage, for example because
a grandparent's origin has moved.
## `box_`
new allocation of the actor, in parent-relative coordinates
<!-- trait ActorExt::fn allocate_align_fill -->
Allocates `self` by taking into consideration the available allocation
area; an alignment factor on either axis; and whether the actor should
fill the allocation on either axis.

The `box_` should contain the available allocation width and height;
if the x1 and y1 members of `ClutterActorBox` are not set to 0, the
allocation will be offset by their value.

This function takes into consideration the geometry request specified by
the `property::Actor::request-mode` property, and the text direction.

This function is useful for fluid layout managers using legacy alignment
flags. Newly written layout managers should use the `property::Actor::x-align`
and `property::Actor::y-align` properties, instead, and just call
`clutter_actor_allocate()` inside their `ClutterActorClass.allocate()`
implementation.
## `box_`
a `ClutterActorBox`, containing the available width and height
## `x_align`
the horizontal alignment, between 0 and 1
## `y_align`
the vertical alignment, between 0 and 1
## `x_fill`
whether the actor should fill horizontally
## `y_fill`
whether the actor should fill vertically
<!-- trait ActorExt::fn apply_relative_transform_to_point -->
Transforms `point` in coordinates relative to the actor into
ancestor-relative coordinates using the relevant transform
stack (i.e. scale, rotation, etc).

If `ancestor` is [`None`] the ancestor will be the [`Stage`][crate::Stage]. In
this case, the coordinates returned will be the coordinates on
the stage before the projection is applied. This is different from
the behaviour of `clutter_actor_apply_transform_to_point()`.
## `ancestor`
A [`Actor`][crate::Actor] ancestor, or [`None`] to use the
 default [`Stage`][crate::Stage]
## `point`
A point as `graphene_point3d_t`

# Returns


## `vertex`
The translated `graphene_point3d_t`
<!-- trait ActorExt::fn apply_transform_to_point -->
Transforms `point` in coordinates relative to the actor
into screen-relative coordinates with the current actor
transformation (i.e. scale, rotation, etc)
## `point`
A point as `graphene_point3d_t`

# Returns


## `vertex`
The translated `graphene_point3d_t`
<!-- trait ActorExt::fn bind_model -->
Binds a `GListModel` to a [`Actor`][crate::Actor].

If the [`Actor`][crate::Actor] was already bound to a `GListModel`, the previous
binding is destroyed.

The existing children of [`Actor`][crate::Actor] are destroyed when setting a
model, and new children are created and added, representing the contents
of the `model`. The [`Actor`][crate::Actor] is updated whenever the `model` changes.
If `model` is [`None`], the [`Actor`][crate::Actor] is left empty.

When a [`Actor`][crate::Actor] is bound to a model, adding and removing children
directly is undefined behaviour.
## `model`
a `GListModel`
## `create_child_func`
a function that creates [`Actor`][crate::Actor] instances
 from the contents of the `model`
## `notify`
function called when unsetting the `model`
<!-- trait ActorExt::fn bind_model_with_properties -->
Binds a `GListModel` to a [`Actor`][crate::Actor].

Unlike `clutter_actor_bind_model()`, this function automatically creates
a child [`Actor`][crate::Actor] of type `child_type`, and binds properties on the
items inside the `model` to the corresponding properties on the child,
for instance:



**⚠️ The following code is in C ⚠️**

```C
  clutter_actor_bind_model_with_properties (actor, model,
                                            MY_TYPE_CHILD_VIEW,
                                            "label", "text", G_BINDING_DEFAULT | G_BINDING_SYNC_CREATE,
                                            "icon", "image", G_BINDING_DEFAULT | G_BINDING_SYNC_CREATE,
                                            "selected", "selected", G_BINDING_BIDIRECTIONAL,
                                            "active", "active", G_BINDING_BIDIRECTIONAL,
                                            NULL);
```

is the equivalent of calling `clutter_actor_bind_model()` with a
`ClutterActorCreateChildFunc` of:



**⚠️ The following code is in C ⚠️**

```C
  ClutterActor *res = g_object_new (MY_TYPE_CHILD_VIEW, NULL);

  g_object_bind_property (item, "label", res, "text", G_BINDING_DEFAULT | G_BINDING_SYNC_CREATE);
  g_object_bind_property (item, "icon", res, "image", G_BINDING_DEFAULT | G_BINDING_SYNC_CREATE);
  g_object_bind_property (item, "selected", res, "selected", G_BINDING_BIDIRECTIONAL);
  g_object_bind_property (item, "active", res, "active", G_BINDING_BIDIRECTIONAL);

  return res;
```

If the [`Actor`][crate::Actor] was already bound to a `GListModel`, the previous
binding is destroyed.

When a [`Actor`][crate::Actor] is bound to a model, adding and removing children
directly is undefined behaviour.

See also: `clutter_actor_bind_model()`
## `model`
a `GListModel`
## `child_type`
the type of [`Actor`][crate::Actor] to use when creating
 children mapping to items inside the `model`
## `first_model_property`
the first property of `model` to bind
<!-- trait ActorExt::fn continue_paint -->
Run the next stage of the paint sequence. This function should only
be called within the implementation of the ‘run’ virtual of a
`ClutterEffect`. It will cause the run method of the next effect to
be applied, or it will paint the actual actor if the current effect
is the last effect in the chain.
<!-- trait ActorExt::fn continue_pick -->
Run the next stage of the pick sequence. This function should only
be called within the implementation of the ‘pick’ virtual of a
`ClutterEffect`. It will cause the run method of the next effect to
be applied, or it will pick the actual actor if the current effect
is the last effect in the chain.
<!-- trait ActorExt::fn create_pango_context -->
Creates a `PangoContext` for the given actor. The `PangoContext`
is already configured using the appropriate font map, resolution
and font options.

See also `clutter_actor_get_pango_context()`.

# Returns

the newly created `PangoContext`.
 Use `g_object_unref()` on the returned value to deallocate its
 resources
<!-- trait ActorExt::fn create_pango_layout -->
Creates a new `PangoLayout` from the same `PangoContext` used
by the [`Actor`][crate::Actor]. The `PangoLayout` is already configured
with the font map, resolution and font options, and the
given `text`.

If you want to keep around a `PangoLayout` created by this
function you will have to connect to the `ClutterBackend::font-changed`
and `ClutterBackend::resolution-changed` signals, and call
`pango_layout_context_changed()` in response to them.
## `text`
the text to set on the `PangoLayout`, or [`None`]

# Returns

the newly created `PangoLayout`.
 Use `g_object_unref()` when done
<!-- trait ActorExt::fn event -->
This function is used to emit an event on the main stage.
You should rarely need to use this function, except for
synthetising events.
## `event`
a `ClutterEvent`
## `capture`
[`true`] if event in in capture phase, [`false`] otherwise.

# Returns

the return value from the signal emission: [`true`]
 if the actor handled the event, or [`false`] if the event was
 not handled
<!-- trait ActorExt::fn abs_allocation_vertices -->
Calculates the transformed screen coordinates of the four corners of
the actor; the returned vertices relate to the `ClutterActorBox`
coordinates as follows:

 - v[0] contains (x1, y1)
 - v[1] contains (x2, y1)
 - v[2] contains (x1, y2)
 - v[3] contains (x2, y2)

# Returns


## `verts`
Pointer to a location of an array
 of 4 `graphene_point3d_t` where to store the result.
<!-- trait ActorExt::fn accessible -->
Returns the accessible object that describes the actor to an
assistive technology.

If no class-specific `AtkObject` implementation is available for the
actor instance in question, it will inherit an `AtkObject`
implementation from the first ancestor class for which such an
implementation is defined.

The documentation of the <ulink
url="http://developer.gnome.org/doc/API/2.0/atk/index.html">ATK`</ulink>`
library contains more information about accessible objects and
their uses.

# Returns

the `AtkObject` associated with `actor`
<!-- trait ActorExt::fn allocation_box -->
Gets the layout box an actor has been assigned. The allocation can
only be assumed valid inside a `paint()` method; anywhere else, it
may be out-of-date.

An allocation does not incorporate the actor's scale or translation;
those transformations do not affect layout, only rendering.

Do not call any of the clutter_actor_get_allocation_*() family
of functions inside the implementation of the `get_preferred_width()`
or `get_preferred_height()` virtual functions.

# Returns


## `box_`
the function fills this in with the actor's allocation
<!-- trait ActorExt::fn child_transform -->
Retrieves the child transformation matrix set using
`clutter_actor_set_child_transform()`; if none is currently set,
the `transform` matrix will be initialized to the identity matrix.

# Returns


## `transform`
a `graphene_matrix_t`
<!-- trait ActorExt::fn content_box -->
Retrieves the bounding box for the [`Content`][crate::Content] of `self`.

The bounding box is relative to the actor's allocation.

If no [`Content`][crate::Content] is set for `self`, or if `self` has not been
allocated yet, then the result is undefined.

The content box is guaranteed to be, at most, as big as the allocation
of the [`Actor`][crate::Actor].

If the [`Content`][crate::Content] used by the actor has a preferred size, then
it is possible to modify the content box by using the
`property::Actor::content-gravity` property.

# Returns


## `box_`
the return location for the bounding
 box for the [`Content`][crate::Content]
<!-- trait ActorExt::fn content_gravity -->
Retrieves the content gravity as set using
`clutter_actor_set_content_gravity()`.

# Returns

the content gravity
<!-- trait ActorExt::fn content_repeat -->
Retrieves the repeat policy for a [`Actor`][crate::Actor] set by
`clutter_actor_set_content_repeat()`.

# Returns

the content repeat policy
<!-- trait ActorExt::fn content_scaling_filters -->
Retrieves the values set using `clutter_actor_set_content_scaling_filters()`.

# Returns


## `min_filter`
return location for the minification
 filter, or [`None`]

## `mag_filter`
return location for the magnification
 filter, or [`None`]
<!-- trait ActorExt::fn default_paint_volume -->
Retrieves the default paint volume for `self`.

This function provides the same `ClutterPaintVolume` that would be
computed by the default implementation inside [`Actor`][crate::Actor] of the
`ClutterActorClass.get_paint_volume()` virtual function.

This function should only be used by [`Actor`][crate::Actor] subclasses that
cannot chain up to the parent implementation when computing their
paint volume.

# Returns

a pointer to the default
 `ClutterPaintVolume`, relative to the [`Actor`][crate::Actor], or [`None`] if
 the actor could not compute a valid paint volume. The returned value
 is not guaranteed to be stable across multiple frames, so if you
 want to retain it, you will need to copy it using
 `clutter_paint_volume_copy()`.
<!-- trait ActorExt::fn effect -->
Retrieves the `ClutterEffect` with the given name in the list
of effects applied to `self`
## `name`
the name of the effect to retrieve

# Returns

a `ClutterEffect` for the given
 name, or [`None`]. The returned `ClutterEffect` is owned by the
 actor and it should not be unreferenced directly
<!-- trait ActorExt::fn effects -->
Retrieves the `ClutterEffect`<!-- -->s applied on `self`, if any

# Returns

a list
 of `ClutterEffect`<!-- -->s, or [`None`]. The elements of the returned
 list are owned by Clutter and they should not be freed. You should
 free the returned list using `g_list_free()` when done
<!-- trait ActorExt::fn layout_manager -->
Retrieves the `ClutterLayoutManager` used by `self`.

# Returns

a pointer to the `ClutterLayoutManager`,
 or [`None`]
<!-- trait ActorExt::fn margin -->
Retrieves all the components of the margin of a [`Actor`][crate::Actor].

# Returns


## `margin`
return location for a `ClutterMargin`
<!-- trait ActorExt::fn offscreen_redirect -->
Retrieves whether to redirect the actor to an offscreen buffer, as
set by `clutter_actor_set_offscreen_redirect()`.

# Returns

the value of the offscreen-redirect property of the actor
<!-- trait ActorExt::fn paint_box -->
Retrieves the paint volume of the passed [`Actor`][crate::Actor], and
transforms it into a 2D bounding box in stage coordinates.

This function is useful to determine the on screen area occupied by
the actor. The box is only an approximation and may often be
considerably larger due to the optimizations used to calculate the
box. The box is never smaller though, so it can reliably be used
for culling.

There are times when a 2D paint box can't be determined, e.g.
because the actor isn't yet parented under a stage or because
the actor is unable to determine a paint volume.

# Returns

[`true`] if a 2D paint box could be determined, else
[`false`].

## `box_`
return location for a `ClutterActorBox`
<!-- trait ActorExt::fn paint_volume -->
Retrieves the paint volume of the passed [`Actor`][crate::Actor], or [`None`]
when a paint volume can't be determined.

The paint volume is defined as the 3D space occupied by an actor
when being painted.

This function will call the `ClutterActorClass.get_paint_volume()`
virtual function of the [`Actor`][crate::Actor] class. Sub-classes of [`Actor`][crate::Actor]
should not usually care about overriding the default implementation,
unless they are, for instance: painting outside their allocation, or
actors with a depth factor (not in terms of `property::Actor::depth` but real
3D depth).

Note: 2D actors overriding `ClutterActorClass.get_paint_volume()`
should ensure that their volume has a depth of 0. (This will be true
as long as you don't call `clutter_paint_volume_set_depth()`.)

# Returns

a pointer to a `ClutterPaintVolume`,
 or [`None`] if no volume could be determined. The returned pointer
 is not guaranteed to be valid across multiple frames; if you want
 to keep it, you will need to copy it using `clutter_paint_volume_copy()`.
<!-- trait ActorExt::fn pango_context -->
Retrieves the `PangoContext` for `self`. The actor's `PangoContext`
is already configured using the appropriate font map, resolution
and font options.

Unlike `clutter_actor_create_pango_context()`, this context is owend
by the [`Actor`][crate::Actor] and it will be updated each time the options
stored by the `ClutterBackend` change.

You can use the returned `PangoContext` to create a `PangoLayout`
and render text using `cogl_pango_show_layout()` to reuse the
glyphs cache also used by Clutter.

# Returns

the `PangoContext` for a [`Actor`][crate::Actor].
 The returned `PangoContext` is owned by the actor and should not be
 unreferenced by the application code
<!-- trait ActorExt::fn request_mode -->
Retrieves the geometry request mode of `self`

# Returns

the request mode for the actor
<!-- trait ActorExt::fn rotation_angle -->
Retrieves the angle of rotation set by `clutter_actor_set_rotation_angle()`.
## `axis`
the axis of the rotation

# Returns

the angle of rotation, in degrees
<!-- trait ActorExt::fn text_direction -->
Retrieves the value set using `clutter_actor_set_text_direction()`

If no text direction has been previously set, the default text
direction, as returned by `clutter_get_default_text_direction()`, will
be returned instead

# Returns

the `ClutterTextDirection` for the actor
<!-- trait ActorExt::fn transform -->
Retrieves the current transformation matrix of a [`Actor`][crate::Actor].

# Returns


## `transform`
a `graphene_matrix_t`
<!-- trait ActorExt::fn transformed_extents -->
Gets the transformed bounding rect of an actor, in pixels relative to the stage.

# Returns


## `rect`
return location for the transformed bounding rect
<!-- trait ActorExt::fn transformed_paint_volume -->
Retrieves the 3D paint volume of an actor like
`clutter_actor_get_paint_volume()` does (Please refer to the
documentation of `clutter_actor_get_paint_volume()` for more
details.) and it additionally transforms the paint volume into the
coordinate space of `relative_to_ancestor`. (Or the stage if [`None`]
is passed for `relative_to_ancestor`)

This can be used by containers that base their paint volume on
the volume of their children. Such containers can query the
transformed paint volume of all of its children and union them
together using `clutter_paint_volume_union()`.
## `relative_to_ancestor`
A [`Actor`][crate::Actor] that is an ancestor of `self`
 (or [`None`] for the stage)

# Returns

a pointer to a `ClutterPaintVolume`,
 or [`None`] if no volume could be determined. The returned pointer is
 not guaranteed to be valid across multiple frames; if you wish to
 keep it, you will have to copy it using `clutter_paint_volume_copy()`.
<!-- trait ActorExt::fn transition -->
Retrieves the `ClutterTransition` of a [`Actor`][crate::Actor] by using the
transition `name`.

Transitions created for animatable properties use the name of the
property itself, for instance the code below:



**⚠️ The following code is in C ⚠️**

```C
  clutter_actor_set_easing_duration (actor, 1000);
  clutter_actor_set_rotation_angle (actor, CLUTTER_Y_AXIS, 360.0);

  transition = clutter_actor_get_transition (actor, "rotation-angle-y");
  g_signal_connect (transition, "stopped",
                    G_CALLBACK (on_transition_stopped),
                    actor);
```

will call the `on_transition_stopped` callback when the transition
is finished.

If you just want to get notifications of the completion of a transition,
you should use the `signal::Actor::transition-stopped` signal, using the
transition name as the signal detail.
## `name`
the name of the transition

# Returns

a `ClutterTransition`, or [`None`] is none
 was found to match the passed name; the returned instance is owned
 by Clutter and it should not be freed
<!-- trait ActorExt::fn needs_expand -->
Checks whether an actor, or any of its children, is set to expand
horizontally or vertically.

This function should only be called by layout managers that can
assign extra space to their children.

If you want to know whether the actor was explicitly set to expand,
use [`is_x_expand()`][Self::is_x_expand()] or [`is_y_expand()`][Self::is_y_expand()].
## `orientation`
the direction of expansion

# Returns

[`true`] if the actor should expand
<!-- trait ActorExt::fn paint -->
Renders the actor to display.

This function should not be called directly by applications.
Call [`queue_redraw()`][Self::queue_redraw()] to queue paints, instead.

This function is context-aware, and will either cause a
regular paint or a pick paint.

This function will call the `ClutterActorClass.paint()` virtual
function.

This function does not paint the actor if the actor is set to 0,
unless it is performing a pick paint.
<!-- trait ActorExt::fn peek_stage_views -->
Retrieves the list of `ClutterStageView`<!-- -->s the actor is being
painted on.

If this function is called during the paint cycle, the list is guaranteed
to be up-to-date, if called outside the paint cycle, the list will
contain the views the actor was painted on last.

The list returned by this function is not updated when the actors
visibility changes: If an actor gets hidden and is not being painted
anymore, this function will return the list of views the actor was
painted on last.

If an actor is not attached to a stage (realized), this function will
always return an empty list.

# Returns

The list of
 `ClutterStageView`<!-- -->s the actor is being painted on. The list and
 its contents are owned by the [`Actor`][crate::Actor] and the list may not be
 freed or modified.
<!-- trait ActorExt::fn pick -->
Asks `self` to perform a pick.
<!-- trait ActorExt::fn pick_box -->
Logs (does a virtual paint of) a rectangle for picking. Note that `box_` is
in the actor's own local coordinates, so is usually {0,0,width,height}
to include the whole actor. That is unless the actor has a shaped input
region in which case you may wish to log the (multiple) smaller rectangles
that make up the input region.
## `pick_context`
The `ClutterPickContext`
## `box_`
A rectangle in the actor's own local coordinates.
<!-- trait ActorExt::fn pick_frame_clock -->
Pick the most suitable frame clock for driving animations for this actor.

The [`Actor`][crate::Actor] used for picking the frame clock is written `out_actor`.
## `out_actor`
a pointer to an [`Actor`][crate::Actor]

# Returns

a `ClutterFrameClock`
<!-- trait ActorExt::fn queue_redraw_with_clip -->
Queues a redraw on `self` limited to a specific, actor-relative
rectangular area.

If `clip` is [`None`] this function is equivalent to
[`queue_redraw()`][Self::queue_redraw()].
## `clip`
a rectangular clip region, or [`None`]
<!-- trait ActorExt::fn remove_effect -->
Removes `effect` from the list of effects applied to `self`

The reference held by `self` on the `ClutterEffect` will be released
## `effect`
a `ClutterEffect`
<!-- trait ActorExt::fn set_allocation -->
Stores the allocation of `self` as defined by `box_`.

This function can only be called from within the implementation of
the `ClutterActorClass.allocate()` virtual function.

The allocation `box_` should have been adjusted to take into account
constraints, alignment, and margin properties.

This function should only be used by subclasses of [`Actor`][crate::Actor]
that wish to store their allocation but cannot chain up to the
parent's implementation; the default implementation of the
`ClutterActorClass.allocate()` virtual function will call this
function.
## `box_`
a `ClutterActorBox`
<!-- trait ActorExt::fn set_child_transform -->
Sets the transformation matrix to be applied to all the children
of `self` prior to their own transformations. The default child
transformation is the identity matrix.

If `transform` is [`None`], the child transform will be unset.

The `property::Actor::child-transform` property is animatable.
## `transform`
a `graphene_matrix_t`, or [`None`]
<!-- trait ActorExt::fn set_content_gravity -->
Sets the gravity of the [`Content`][crate::Content] used by `self`.

See the description of the `property::Actor::content-gravity` property for
more information.

The `property::Actor::content-gravity` property is animatable.
## `gravity`
the `ClutterContentGravity`
<!-- trait ActorExt::fn set_content_repeat -->
Sets the policy for repeating the `property::Actor::content` of a
[`Actor`][crate::Actor]. The behaviour is deferred to the [`Content`][crate::Content]
implementation.
## `repeat`
the repeat policy
<!-- trait ActorExt::fn set_content_scaling_filters -->
Sets the minification and magnification filter to be applied when
scaling the `property::Actor::content` of a [`Actor`][crate::Actor].

The `property::Actor::minification-filter` will be used when reducing
the size of the content; the `property::Actor::magnification-filter`
will be used when increasing the size of the content.
## `min_filter`
the minification filter for the content
## `mag_filter`
the magnification filter for the content
<!-- trait ActorExt::fn set_layout_manager -->
Sets the `ClutterLayoutManager` delegate object that will be used to
lay out the children of `self`.

The [`Actor`][crate::Actor] will take a reference on the passed `manager` which
will be released either when the layout manager is removed, or when
the actor is destroyed.
## `manager`
a `ClutterLayoutManager`, or [`None`] to unset it
<!-- trait ActorExt::fn set_margin -->
Sets all the components of the margin of a [`Actor`][crate::Actor].
## `margin`
a `ClutterMargin`
<!-- trait ActorExt::fn set_offscreen_redirect -->
Defines the circumstances where the actor should be redirected into
an offscreen image. The offscreen image is used to flatten the
actor into a single image while painting for two main reasons.
Firstly, when the actor is painted a second time without any of its
contents changing it can simply repaint the cached image without
descending further down the actor hierarchy. Secondly, it will make
the opacity look correct even if there are overlapping primitives
in the actor.

Caching the actor could in some cases be a performance win and in
some cases be a performance lose so it is important to determine
which value is right for an actor before modifying this value. For
example, there is never any reason to flatten an actor that is just
a single texture (such as a `ClutterTexture`) because it is
effectively already cached in an image so the offscreen would be
redundant. Also if the actor contains primitives that are far apart
with a large transparent area in the middle (such as a large
CluterGroup with a small actor in the top left and a small actor in
the bottom right) then the cached image will contain the entire
image of the large area and the paint will waste time blending all
of the transparent pixels in the middle.

The default method of implementing opacity on a container simply
forwards on the opacity to all of the children. If the children are
overlapping then it will appear as if they are two separate glassy
objects and there will be a break in the color where they
overlap. By redirecting to an offscreen buffer it will be as if the
two opaque objects are combined into one and then made transparent
which is usually what is expected.

The image below demonstrates the difference between redirecting and
not. The image shows two Clutter groups, each containing a red and
a green rectangle which overlap. The opacity on the group is set to
128 (which is 50%). When the offscreen redirect is not used, the
red rectangle can be seen through the blue rectangle as if the two
rectangles were separately transparent. When the redirect is used
the group as a whole is transparent instead so the red rectangle is
not visible where they overlap.

<figure id="offscreen-redirect">
 `<title>`Sample of using an offscreen redirect for transparency`</title>`
 <graphic fileref="offscreen-redirect.png" format="PNG"/>
`</figure>`

The default value for this property is 0, so we effectively will
never redirect an actor offscreen by default. This means that there
are times that transparent actors may look glassy as described
above. The reason this is the default is because there is a
performance trade off between quality and performance here. In many
cases the default form of glassy opacity looks good enough, but if
it's not you will need to set the
`CLUTTER_OFFSCREEN_REDIRECT_AUTOMATIC_FOR_OPACITY` flag to enable
redirection for opacity.

Custom actors that don't contain any overlapping primitives are
recommended to override the `has_overlaps()` virtual to return [`false`]
for maximum efficiency.
## `redirect`
New offscreen redirect flags for the actor.
<!-- trait ActorExt::fn set_request_mode -->
Sets the geometry request mode of `self`.

The `mode` determines the order for invoking
[`preferred_width()`][Self::preferred_width()] and
[`preferred_height()`][Self::preferred_height()]
## `mode`
the request mode
<!-- trait ActorExt::fn set_rotation_angle -->
Sets the `angle` of rotation of a [`Actor`][crate::Actor] on the given `axis`.

This function is a convenience for setting the rotation properties
`property::Actor::rotation-angle-x`, `property::Actor::rotation-angle-y`,
and `property::Actor::rotation-angle-z`.

The center of rotation is established by the `property::Actor::pivot-point`
property.
## `axis`
the axis to set the angle one
## `angle`
the angle of rotation, in degrees
<!-- trait ActorExt::fn set_text_direction -->
Sets the `ClutterTextDirection` for an actor

The passed text direction must not be `CLUTTER_TEXT_DIRECTION_DEFAULT`

If `self` implements [`Container`][crate::Container] then this function will recurse
inside all the children of `self` (including the internal ones).

Composite actors not implementing [`Container`][crate::Container], or actors requiring
special handling when the text direction changes, should connect to
the `GObject::notify` signal for the `property::Actor::text-direction` property
## `text_dir`
the text direction for `self`
<!-- trait ActorExt::fn set_transform -->
Overrides the transformations of a [`Actor`][crate::Actor] with a custom
matrix, which will be applied relative to the origin of the
actor's allocation and to the actor's pivot point.

The `property::Actor::transform` property is animatable.
## `transform`
a `graphene_matrix_t`, or [`None`] to
 unset a custom transformation
<!-- trait ActorExt::fn should_pick -->
Should be called inside the implementation of the
`signal::Actor::pick` virtual function in order to check whether
the actor should be picked or not.

This function should never be called directly by applications.
## `pick_context`
a `ClutterPickContext`

# Returns

[`true`] if the actor should be picked, [`false`] otherwise
<!-- trait ActorExt::fn connect_button_press_event -->
The ::button-press-event signal is emitted each time a mouse button
is pressed on `actor`.
## `event`
a `ClutterButtonEvent`

# Returns

[`true`] if the event has been handled by the actor,
 or [`false`] to continue the emission.
<!-- trait ActorExt::fn connect_button_release_event -->
The ::button-release-event signal is emitted each time a mouse button
is released on `actor`.
## `event`
a `ClutterButtonEvent`

# Returns

[`true`] if the event has been handled by the actor,
 or [`false`] to continue the emission.
<!-- trait ActorExt::fn connect_captured_event -->
The ::captured-event signal is emitted when an event is captured
by Clutter. This signal will be emitted starting from the top-level
container (the [`Stage`][crate::Stage]) to the actor which received the event
going down the hierarchy. This signal can be used to intercept every
event before the specialized events (like
ClutterActor::button-press-event or ::key-released-event) are
emitted.
## `event`
a `ClutterEvent`

# Returns

[`true`] if the event has been handled by the actor,
 or [`false`] to continue the emission.
<!-- trait ActorExt::fn connect_enter_event -->
The ::enter-event signal is emitted when the pointer enters the `actor`
## `event`
a `ClutterCrossingEvent`

# Returns

[`true`] if the event has been handled by the actor,
 or [`false`] to continue the emission.
<!-- trait ActorExt::fn connect_event -->
The ::event signal is emitted each time an event is received
by the `actor`. This signal will be emitted on every actor,
following the hierarchy chain, until it reaches the top-level
container (the [`Stage`][crate::Stage]).
## `event`
a `ClutterEvent`

# Returns

[`true`] if the event has been handled by the actor,
 or [`false`] to continue the emission.
<!-- trait ActorExt::fn connect_leave_event -->
The ::leave-event signal is emitted when the pointer leaves the `actor`.
## `event`
a `ClutterCrossingEvent`

# Returns

[`true`] if the event has been handled by the actor,
 or [`false`] to continue the emission.
<!-- trait ActorExt::fn connect_motion_event -->
The ::motion-event signal is emitted each time the mouse pointer is
moved over `actor`.
## `event`
a `ClutterMotionEvent`

# Returns

[`true`] if the event has been handled by the actor,
 or [`false`] to continue the emission.
<!-- trait ActorExt::fn connect_pick -->
The ::pick signal is emitted each time an actor is being painted
in "pick mode". The pick mode is used to identify the actor during
the event handling phase, or by `clutter_stage_get_actor_at_pos()`.

Subclasses of [`Actor`][crate::Actor] should override the class signal handler
and paint themselves in that function.

It is possible to connect a handler to the ::pick signal in order
to set up some custom aspect of a paint in pick mode.

# Deprecated since 1.12

Override the `ClutterActorClass.pick` virtual function
 instead.
## `pick_context`
a `ClutterPickContext`
<!-- trait ActorExt::fn connect_scroll_event -->
The ::scroll-event signal is emitted each time the mouse is
scrolled on `actor`
## `event`
a `ClutterScrollEvent`

# Returns

[`true`] if the event has been handled by the actor,
 or [`false`] to continue the emission.
<!-- trait ActorExt::fn connect_touch_event -->
The ::touch-event signal is emitted each time a touch
begin/end/update/cancel event.
## `event`
a `ClutterEvent`

# Returns

`CLUTTER_EVENT_STOP` if the event has been handled by
 the actor, or `CLUTTER_EVENT_PROPAGATE` to continue the emission.
<!-- trait ActorExt::fn allocation -->
The allocation for the actor, in pixels

This is property is read-only, but you might monitor it to know when an
actor moves or resizes
<!-- trait ActorExt::fn clip_rect -->
The visible region of the actor, in actor-relative coordinates,
expressed as a `graphene_rect_t`.

Setting this property to [`None`] will unset the existing clip.

Setting this property will change the `property::Actor::has-clip`
property as a side effect.
<!-- trait ActorExt::fn set_clip_rect -->
The visible region of the actor, in actor-relative coordinates,
expressed as a `graphene_rect_t`.

Setting this property to [`None`] will unset the existing clip.

Setting this property will change the `property::Actor::has-clip`
property as a side effect.
<!-- trait ActorExt::fn set_effect -->
Adds `ClutterEffect` to the list of effects be applied on a [`Actor`][crate::Actor]
<!-- trait ActorExt::fn fixed_position_set -->
This flag controls whether the `property::Actor::fixed-x` and
`property::Actor::fixed-y` properties are used
<!-- trait ActorExt::fn reactive -->
Whether the actor is reactive to events or not

Only reactive actors will emit event-related signals
<!-- trait ActorExt::fn get_property_request_mode -->
Request mode for the [`Actor`][crate::Actor]. The request mode determines the
type of geometry management used by the actor, either height for width
(the default) or width for height.

For actors implementing height for width, the parent container should get
the preferred width first, and then the preferred height for that width.

For actors implementing width for height, the parent container should get
the preferred height first, and then the preferred width for that height.

For instance:



**⚠️ The following code is in C ⚠️**

```C
  ClutterRequestMode mode;
  gfloat natural_width, min_width;
  gfloat natural_height, min_height;

  mode = clutter_actor_get_request_mode (child);
  if (mode == CLUTTER_REQUEST_HEIGHT_FOR_WIDTH)
    {
      clutter_actor_get_preferred_width (child, -1,
                                         &min_width,
                                         &natural_width);
      clutter_actor_get_preferred_height (child, natural_width,
                                          &min_height,
                                          &natural_height);
    }
  else if (mode == CLUTTER_REQUEST_WIDTH_FOR_HEIGHT)
    {
      clutter_actor_get_preferred_height (child, -1,
                                          &min_height,
                                          &natural_height);
      clutter_actor_get_preferred_width (child, natural_height,
                                         &min_width,
                                         &natural_width);
    }
  else if (mode == CLUTTER_REQUEST_CONTENT_SIZE)
    {
      ClutterContent *content = clutter_actor_get_content (child);

      min_width, min_height = 0;
      natural_width = natural_height = 0;

      if (content != NULL)
        clutter_content_get_preferred_size (content, &natural_width, &natural_height);
    }
```

will retrieve the minimum and natural width and height depending on the
preferred request mode of the [`Actor`][crate::Actor] "child".

The [`preferred_size()`][Self::preferred_size()] function will implement this
check for you.
<!-- trait ActorExt::fn set_property_request_mode -->
Request mode for the [`Actor`][crate::Actor]. The request mode determines the
type of geometry management used by the actor, either height for width
(the default) or width for height.

For actors implementing height for width, the parent container should get
the preferred width first, and then the preferred height for that width.

For actors implementing width for height, the parent container should get
the preferred height first, and then the preferred width for that height.

For instance:



**⚠️ The following code is in C ⚠️**

```C
  ClutterRequestMode mode;
  gfloat natural_width, min_width;
  gfloat natural_height, min_height;

  mode = clutter_actor_get_request_mode (child);
  if (mode == CLUTTER_REQUEST_HEIGHT_FOR_WIDTH)
    {
      clutter_actor_get_preferred_width (child, -1,
                                         &min_width,
                                         &natural_width);
      clutter_actor_get_preferred_height (child, natural_width,
                                          &min_height,
                                          &natural_height);
    }
  else if (mode == CLUTTER_REQUEST_WIDTH_FOR_HEIGHT)
    {
      clutter_actor_get_preferred_height (child, -1,
                                          &min_height,
                                          &natural_height);
      clutter_actor_get_preferred_width (child, natural_height,
                                         &min_width,
                                         &natural_width);
    }
  else if (mode == CLUTTER_REQUEST_CONTENT_SIZE)
    {
      ClutterContent *content = clutter_actor_get_content (child);

      min_width, min_height = 0;
      natural_width = natural_height = 0;

      if (content != NULL)
        clutter_content_get_preferred_size (content, &natural_width, &natural_height);
    }
```

will retrieve the minimum and natural width and height depending on the
preferred request mode of the [`Actor`][crate::Actor] "child".

The [`preferred_size()`][Self::preferred_size()] function will implement this
check for you.
<!-- trait ActorExt::fn get_property_text_direction -->
The direction of the text inside a [`Actor`][crate::Actor].
<!-- trait ActorExt::fn set_property_text_direction -->
The direction of the text inside a [`Actor`][crate::Actor].
<!-- trait ActorExt::fn x_expand -->
Whether a layout manager should assign more space to the actor on
the X axis.
<!-- trait ActorExt::fn y_expand -->
Whether a layout manager should assign more space to the actor on
the Y axis.
<!-- enum ActorAlign::variant Fill -->
Stretch to cover the whole allocated space
<!-- enum ActorAlign::variant Start -->
Snap to left or top side, leaving space
 to the right or bottom. For horizontal layouts, in right-to-left
 locales this should be reversed.
<!-- enum ActorAlign::variant Center -->
Center the actor inside the allocation
<!-- enum ActorAlign::variant End -->
Snap to right or bottom side, leaving space
 to the left or top. For horizontal layouts, in right-to-left locales
 this should be reversed.
<!-- struct ActorFlags -->
Flags used to signal the state of an actor.
<!-- struct ActorFlags::const MAPPED -->
the actor will be painted (is visible, and inside
 a toplevel, and all parents visible)
<!-- struct ActorFlags::const REALIZED -->
the resources associated to the actor have been
 allocated
<!-- struct ActorFlags::const REACTIVE -->
the actor 'reacts' to mouse events emitting event
 signals
<!-- struct ActorFlags::const VISIBLE -->
the actor has been shown by the application program
<!-- struct ActorFlags::const NO_LAYOUT -->
the actor provides an explicit layout management
 policy for its children; this flag will prevent Clutter from automatic
 queueing of relayout and will defer all layouting to the actor itself
<!-- struct ActorMeta -->


This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`ActorMetaExt`][trait@crate::prelude::ActorMetaExt]
<!-- trait ActorMetaExt::fn enabled -->
Whether or not the [`ActorMeta`][crate::ActorMeta] is enabled
<!-- struct Animatable -->


# Implements

[`AnimatableExt`][trait@crate::prelude::AnimatableExt]
<!-- trait AnimatableExt::fn find_property -->
Finds the `GParamSpec` for `property_name`
## `property_name`
the name of the animatable property to find

# Returns

The `GParamSpec` for the given property
 or [`None`]
<!-- trait AnimatableExt::fn initial_state -->
Retrieves the current state of `property_name` and sets `value` with it
## `property_name`
the name of the animatable property to retrieve
## `value`
a `GValue` initialized to the type of the property to retrieve
<!-- trait AnimatableExt::fn interpolate_value -->
Asks a [`Animatable`][crate::Animatable] implementation to interpolate a
a named property between the initial and final values of
a `ClutterInterval`, using `progress` as the interpolation
value, and store the result inside `value`.

This function should be used for every property animation
involving [`Animatable`][crate::Animatable]<!-- -->s.

This function replaces `clutter_animatable_animate_property()`.
## `property_name`
the name of the property to interpolate
## `interval`
a `ClutterInterval` with the animation range
## `progress`
the progress to use to interpolate between the
 initial and final values of the `interval`

# Returns

[`true`] if the interpolation was successful,
 and [`false`] otherwise

## `value`
return location for an initialized `GValue`
 using the same type of the `interval`
<!-- trait AnimatableExt::fn set_final_state -->
Sets the current state of `property_name` to `value`
## `property_name`
the name of the animatable property to set
## `value`
the value of the animatable property to set
<!-- enum AnimationMode::variant CustomMode -->
custom progress function
<!-- enum AnimationMode::variant Linear -->
linear tweening
<!-- enum AnimationMode::variant EaseInQuad -->
quadratic tweening
<!-- enum AnimationMode::variant EaseOutQuad -->
quadratic tweening, inverse of
 [`EaseInQuad`][Self::EaseInQuad]
<!-- enum AnimationMode::variant EaseInOutQuad -->
quadratic tweening, combininig
 [`EaseInQuad`][Self::EaseInQuad] and [`EaseOutQuad`][Self::EaseOutQuad]
<!-- enum AnimationMode::variant EaseInCubic -->
cubic tweening
<!-- enum AnimationMode::variant EaseOutCubic -->
cubic tweening, inverse of
 [`EaseInCubic`][Self::EaseInCubic]
<!-- enum AnimationMode::variant EaseInOutCubic -->
cubic tweening, combining
 [`EaseInCubic`][Self::EaseInCubic] and [`EaseOutCubic`][Self::EaseOutCubic]
<!-- enum AnimationMode::variant EaseInQuart -->
quartic tweening
<!-- enum AnimationMode::variant EaseOutQuart -->
quartic tweening, inverse of
 [`EaseInQuart`][Self::EaseInQuart]
<!-- enum AnimationMode::variant EaseInOutQuart -->
quartic tweening, combining
 [`EaseInQuart`][Self::EaseInQuart] and [`EaseOutQuart`][Self::EaseOutQuart]
<!-- enum AnimationMode::variant EaseInQuint -->
quintic tweening
<!-- enum AnimationMode::variant EaseOutQuint -->
quintic tweening, inverse of
 [`EaseInQuint`][Self::EaseInQuint]
<!-- enum AnimationMode::variant EaseInOutQuint -->
fifth power tweening, combining
 [`EaseInQuint`][Self::EaseInQuint] and [`EaseOutQuint`][Self::EaseOutQuint]
<!-- enum AnimationMode::variant EaseInSine -->
sinusoidal tweening
<!-- enum AnimationMode::variant EaseOutSine -->
sinusoidal tweening, inverse of
 [`EaseInSine`][Self::EaseInSine]
<!-- enum AnimationMode::variant EaseInOutSine -->
sine wave tweening, combining
 [`EaseInSine`][Self::EaseInSine] and [`EaseOutSine`][Self::EaseOutSine]
<!-- enum AnimationMode::variant EaseInExpo -->
exponential tweening
<!-- enum AnimationMode::variant EaseOutExpo -->
exponential tweening, inverse of
 [`EaseInExpo`][Self::EaseInExpo]
<!-- enum AnimationMode::variant EaseInOutExpo -->
exponential tweening, combining
 [`EaseInExpo`][Self::EaseInExpo] and [`EaseOutExpo`][Self::EaseOutExpo]
<!-- enum AnimationMode::variant EaseInCirc -->
circular tweening
<!-- enum AnimationMode::variant EaseOutCirc -->
circular tweening, inverse of
 [`EaseInCirc`][Self::EaseInCirc]
<!-- enum AnimationMode::variant EaseInOutCirc -->
circular tweening, combining
 [`EaseInCirc`][Self::EaseInCirc] and [`EaseOutCirc`][Self::EaseOutCirc]
<!-- enum AnimationMode::variant EaseInElastic -->
elastic tweening, with offshoot on start
<!-- enum AnimationMode::variant EaseOutElastic -->
elastic tweening, with offshoot on end
<!-- enum AnimationMode::variant EaseInOutElastic -->
elastic tweening with offshoot on both ends
<!-- enum AnimationMode::variant EaseInBack -->
overshooting cubic tweening, with
 backtracking on start
<!-- enum AnimationMode::variant EaseOutBack -->
overshooting cubic tweening, with
 backtracking on end
<!-- enum AnimationMode::variant EaseInOutBack -->
overshooting cubic tweening, with
 backtracking on both ends
<!-- enum AnimationMode::variant EaseInBounce -->
exponentially decaying parabolic (bounce)
 tweening, with bounce on start
<!-- enum AnimationMode::variant EaseOutBounce -->
exponentially decaying parabolic (bounce)
 tweening, with bounce on end
<!-- enum AnimationMode::variant EaseInOutBounce -->
exponentially decaying parabolic (bounce)
 tweening, with bounce on both ends
<!-- enum AnimationMode::variant Steps -->
parametrized step function; see `clutter_timeline_set_step_progress()`
 for further details. (Since 1.12)
<!-- enum AnimationMode::variant StepStart -->
equivalent to [`Steps`][Self::Steps] with a number of steps
 equal to 1, and a step mode of `CLUTTER_STEP_MODE_START`. (Since 1.12)
<!-- enum AnimationMode::variant StepEnd -->
equivalent to [`Steps`][Self::Steps] with a number of steps
 equal to 1, and a step mode of `CLUTTER_STEP_MODE_END`. (Since 1.12)
<!-- enum AnimationMode::variant CubicBezier -->
cubic bezier between (0, 0) and (1, 1) with two
 control points; see `clutter_timeline_set_cubic_bezier_progress()`. (Since 1.12)
<!-- enum AnimationMode::variant Ease -->
equivalent to [`CubicBezier`][Self::CubicBezier] with control points
 in (0.25, 0.1) and (0.25, 1.0). (Since 1.12)
<!-- enum AnimationMode::variant EaseIn -->
equivalent to [`CubicBezier`][Self::CubicBezier] with control points
 in (0.42, 0) and (1.0, 1.0). (Since 1.12)
<!-- enum AnimationMode::variant EaseOut -->
equivalent to [`CubicBezier`][Self::CubicBezier] with control points
 in (0, 0) and (0.58, 1.0). (Since 1.12)
<!-- enum AnimationMode::variant EaseInOut -->
equivalent to [`CubicBezier`][Self::CubicBezier] with control points
 in (0.42, 0) and (0.58, 1.0). (Since 1.12)
<!-- enum AnimationMode::variant AnimationLast -->
last animation mode, used as a guard for
 registered global alpha functions
<!-- struct Color -->
Color representation.
<!-- impl Color::fn copy -->
Makes a copy of the color structure. The result must be
freed using `clutter_color_free()`.

# Returns

an allocated copy of `self`.
<!-- impl Color::fn equal -->
Compares two [`Color`][crate::Color]<!-- -->s and checks if they are the same.

This function can be passed to `g_hash_table_new()` as the `key_equal_func`
parameter, when using [`Color`][crate::Color]<!-- -->s as keys in a `GHashTable`.
## `v2`
a [`Color`][crate::Color]

# Returns

[`true`] if the two colors are the same.
<!-- impl Color::fn free -->
Frees a color structure created with `clutter_color_copy()`.
<!-- impl Color::fn hash -->
Converts a [`Color`][crate::Color] to a hash value.

This function can be passed to `g_hash_table_new()` as the `hash_func`
parameter, when using [`Color`][crate::Color]<!-- -->s as keys in a `GHashTable`.

# Returns

a hash value corresponding to the color
<!-- impl Color::fn to_string -->
Returns a textual specification of `self` in the hexadecimal form
`<literal>`&num;rrggbbaa`</literal>`, where `<literal>`r`</literal>`,
`<literal>`g`</literal>`, `<literal>`b`</literal>` and `<literal>`a`</literal>` are
hexadecimal digits representing the red, green, blue and alpha components
respectively.

# Returns

a newly-allocated text string
<!-- struct Constraint -->
The [`Constraint`][crate::Constraint] structure contains only
private data and should be accessed using the provided API

This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`ConstraintExt`][trait@crate::prelude::ConstraintExt], [`ActorMetaExt`][trait@crate::prelude::ActorMetaExt]
<!-- trait ConstraintExt::fn update_preferred_size -->
Asks the `self` to update the size request of a [`Actor`][crate::Actor].
## `actor`
a [`Actor`][crate::Actor]
## `direction`
a `ClutterOrientation`
## `for_size`
the size in the opposite direction
## `minimum_size`
the minimum size to modify
## `natural_size`
the natural size to modify
<!-- struct Container -->
[`Container`][crate::Container] is an opaque structure whose members cannot be directly
accessed

# Implements

[`ContainerExt`][trait@crate::prelude::ContainerExt]
<!-- impl Container::fn class_find_child_property -->
Looks up the `GParamSpec` for a child property of `klass`.
## `klass`
a `GObjectClass` implementing the [`Container`][crate::Container] interface.
## `property_name`
a property name.

# Returns

The `GParamSpec` for the property or [`None`]
 if no such property exist.
<!-- impl Container::fn class_list_child_properties -->
Returns an array of `GParamSpec` for all child properties.
## `klass`
a `GObjectClass` implementing the [`Container`][crate::Container] interface.

# Returns

an array
 of `GParamSpec`<!-- -->s which should be freed after use.
<!-- trait ContainerExt::fn add -->
Adds a list of [`Actor`][crate::Actor]<!-- -->s to `self`. Each time and
actor is added, the "actor-added" signal is emitted. Each actor should
be parented to `self`, which takes a reference on the actor. You
cannot add a [`Actor`][crate::Actor] to more than one [`Container`][crate::Container].

This function will call `ClutterContainerIface.add()`, which is a
deprecated virtual function. The default implementation will
call [`ActorExt::add_child()`][crate::prelude::ActorExt::add_child()].

# Deprecated since 1.10

Use [`ActorExt::add_child()`][crate::prelude::ActorExt::add_child()] instead.
## `first_actor`
the first [`Actor`][crate::Actor] to add
<!-- trait ContainerExt::fn child_get -->
Gets `self` specific properties of an actor.

In general, a copy is made of the property contents and the caller is
responsible for freeing the memory in the appropriate manner for the type, for
instance by calling `g_free()` or `g_object_unref()`.
## `actor`
a [`Actor`][crate::Actor] that is a child of `self`.
## `first_prop`
name of the first property to be set.
<!-- trait ContainerExt::fn child_get_property -->
Gets a container specific property of a child of `self`, In general,
a copy is made of the property contents and the caller is responsible for
freeing the memory by calling `g_value_unset()`.

Note that `clutter_container_child_set_property()` is really intended for
language bindings, `clutter_container_child_set()` is much more convenient
for C programming.
## `child`
a [`Actor`][crate::Actor] that is a child of `self`.
## `property`
the name of the property to set.
## `value`
the value.
<!-- trait ContainerExt::fn child_notify -->
Calls the `ClutterContainerIface.child_notify()` virtual function
of [`Container`][crate::Container]. The default implementation will emit the
`signal::Container::child-notify` signal.
## `child`
a [`Actor`][crate::Actor]
## `pspec`
a `GParamSpec`
<!-- trait ContainerExt::fn child_set -->
Sets container specific properties on the child of a container.
## `actor`
a [`Actor`][crate::Actor] that is a child of `self`.
## `first_prop`
name of the first property to be set.
<!-- trait ContainerExt::fn child_set_property -->
Sets a container-specific property on a child of `self`.
## `child`
a [`Actor`][crate::Actor] that is a child of `self`.
## `property`
the name of the property to set.
## `value`
the value.
<!-- trait ContainerExt::fn child_meta -->
Retrieves the `ClutterChildMeta` which contains the data about the
`self` specific state for `actor`.
## `actor`
a [`Actor`][crate::Actor] that is a child of `self`.

# Returns

the `ClutterChildMeta` for the `actor` child
 of `self` or [`None`] if the specifiec actor does not exist or the
 container is not configured to provide `ClutterChildMeta`<!-- -->s
<!-- trait ContainerExt::fn remove -->
Removes a [`None`] terminated list of [`Actor`][crate::Actor]<!-- -->s from
`self`. Each actor should be unparented, so if you want to keep it
around you must hold a reference to it yourself, using `g_object_ref()`.
Each time an actor is removed, the "actor-removed" signal is
emitted by `self`.

This function will call `ClutterContainerIface.remove()`, which is a
deprecated virtual function. The default implementation will call
[`ActorExt::remove_child()`][crate::prelude::ActorExt::remove_child()].

# Deprecated since 1.10

Use [`ActorExt::remove_child()`][crate::prelude::ActorExt::remove_child()] instead.
## `first_actor`
first [`Actor`][crate::Actor] to remove
<!-- trait ContainerExt::fn connect_child_notify -->
The ::child-notify signal is emitted each time a property is
being set through the `clutter_container_child_set()` and
`clutter_container_child_set_property()` calls.
## `actor`
the child that has had a property set
## `pspec`
the `GParamSpec` of the property set
<!-- struct Content -->


# Implements

[`ContentExt`][trait@crate::prelude::ContentExt]
<!-- struct EventFlags -->
Flags for the `ClutterEvent`
<!-- struct EventFlags::const NONE -->
No flag set
<!-- struct EventFlags::const FLAG_SYNTHETIC -->
Synthetic event
<!-- struct EventFlags::const FLAG_REPEATED -->
Auto-repeated event
<!-- enum EventType::variant Nothing -->
Empty event
<!-- enum EventType::variant KeyPress -->
Key press event
<!-- enum EventType::variant KeyRelease -->
Key release event
<!-- enum EventType::variant Motion -->
Pointer motion event
<!-- enum EventType::variant Enter -->
Actor enter event
<!-- enum EventType::variant Leave -->
Actor leave event
<!-- enum EventType::variant ButtonPress -->
Pointer button press event
<!-- enum EventType::variant ButtonRelease -->
Pointer button release event
<!-- enum EventType::variant Scroll -->
Pointer scroll event
<!-- enum EventType::variant TouchBegin -->
A new touch event sequence has started;
 event added in 1.10
<!-- enum EventType::variant TouchUpdate -->
A touch event sequence has been updated;
 event added in 1.10
<!-- enum EventType::variant TouchEnd -->
A touch event sequence has finished;
 event added in 1.10
<!-- enum EventType::variant TouchCancel -->
A touch event sequence has been canceled;
 event added in 1.10
<!-- enum EventType::variant TouchpadPinch -->
A pinch gesture event, the current state is
 determined by its phase field; event added in 1.24
<!-- enum EventType::variant TouchpadSwipe -->
A swipe gesture event, the current state is
 determined by its phase field; event added in 1.24
<!-- enum EventType::variant ProximityIn -->
A tool entered in proximity to a tablet;
 event added in 1.28
<!-- enum EventType::variant ProximityOut -->
A tool left from the proximity area of a tablet;
 event added in 1.28
<!-- enum EventType::variant EventLast -->
Marks the end of the [`EventType`][crate::EventType] enumeration;
 added in 1.10
<!-- struct InputDevice -->
Generic representation of an input device. The actual contents of this
structure depend on the backend used.
<!-- impl InputDevice::fn device_mode -->
Retrieves the `ClutterInputMode` of `self`.

# Returns

the device mode
<!-- impl InputDevice::fn device_type -->
Retrieves the type of `self`

# Returns

the type of the device
<!-- impl InputDevice::fn seat -->
Returns the seat the device belongs to

# Returns

the device seat
<!-- impl InputDevice::fn sequence_get_grabbed_actor -->
Retrieves a pointer to the [`Actor`][crate::Actor] currently grabbing the
touch events coming from `self` given the `sequence`.
## `sequence`
a `ClutterEventSequence`

# Returns

a [`Actor`][crate::Actor], or [`None`]
<!-- impl InputDevice::fn sequence_grab -->
Acquires a grab on `actor` for the given `self` and the given touch
`sequence`.

Any touch event coming from `self` and from `sequence` will be
delivered to `actor`, bypassing the usual event delivery mechanism,
until the grab is released by calling
`clutter_input_device_sequence_ungrab()`.

The grab is client-side: even if the windowing system used by the Clutter
backend has the concept of "device grabs", Clutter will not use them.
## `sequence`
a `ClutterEventSequence`
## `actor`
a [`Actor`][crate::Actor]
<!-- impl InputDevice::fn sequence_ungrab -->
Releases the grab on the `self` for the given `sequence`, if one is
in place.
## `sequence`
a `ClutterEventSequence`
<!-- impl InputDevice::fn backend -->
The `ClutterBackend` that created the device.
<!-- impl InputDevice::fn set_backend -->
The `ClutterBackend` that created the device.
<!-- impl InputDevice::fn set_device_type -->
The type of the device
<!-- impl InputDevice::fn set_has_cursor -->
Whether the device has an on screen cursor following its movement.
<!-- impl InputDevice::fn set_name -->
The name of the device
<!-- impl InputDevice::fn set_product_id -->
Product ID of this device.
<!-- impl InputDevice::fn set_seat -->
The `ClutterSeat` instance which owns the device
<!-- impl InputDevice::fn set_vendor_id -->
Vendor ID of this device.
<!-- struct InputFocus -->


This is an Abstract Base Class, you cannot instantiate it.

# Implements

[`InputFocusExt`][trait@crate::prelude::InputFocusExt]
<!-- struct KeyEvent -->
Key event
<!-- struct ModifierType -->
Masks applied to a `ClutterEvent` by modifiers.

Note that Clutter may add internal values to events which include
reserved values such as [`MODIFIER_RESERVED_13_MASK`][Self::MODIFIER_RESERVED_13_MASK]. Your code
should preserve and ignore them. You can use [`MODIFIER_MASK`][Self::MODIFIER_MASK] to
remove all reserved values.
<!-- struct ModifierType::const SHIFT_MASK -->
Mask applied by the Shift key
<!-- struct ModifierType::const LOCK_MASK -->
Mask applied by the Caps Lock key
<!-- struct ModifierType::const CONTROL_MASK -->
Mask applied by the Control key
<!-- struct ModifierType::const MOD1_MASK -->
Mask applied by the first Mod key
<!-- struct ModifierType::const MOD2_MASK -->
Mask applied by the second Mod key
<!-- struct ModifierType::const MOD3_MASK -->
Mask applied by the third Mod key
<!-- struct ModifierType::const MOD4_MASK -->
Mask applied by the fourth Mod key
<!-- struct ModifierType::const MOD5_MASK -->
Mask applied by the fifth Mod key
<!-- struct ModifierType::const BUTTON1_MASK -->
Mask applied by the first pointer button
<!-- struct ModifierType::const BUTTON2_MASK -->
Mask applied by the second pointer button
<!-- struct ModifierType::const BUTTON3_MASK -->
Mask applied by the third pointer button
<!-- struct ModifierType::const BUTTON4_MASK -->
Mask applied by the fourth pointer button
<!-- struct ModifierType::const BUTTON5_MASK -->
Mask applied by the fifth pointer button
<!-- struct ModifierType::const SUPER_MASK -->
Mask applied by the Super key
<!-- struct ModifierType::const HYPER_MASK -->
Mask applied by the Hyper key
<!-- struct ModifierType::const META_MASK -->
Mask applied by the Meta key
<!-- struct ModifierType::const RELEASE_MASK -->
Mask applied during release
<!-- struct ModifierType::const MODIFIER_MASK -->
A mask covering all modifier types
<!-- struct Scriptable -->
[`Scriptable`][crate::Scriptable] is an opaque structure whose members cannot be directly
accessed

# Implements

[`ScriptableExt`][trait@crate::prelude::ScriptableExt]
<!-- trait ScriptableExt::fn parse_custom_node -->
Parses the passed JSON node. The implementation must set the type
of the passed `GValue` pointer using `g_value_init()`.
## `script`
the `ClutterScript` creating the scriptable instance
## `value`
the generic value to be set
## `name`
the name of the node
## `node`
the JSON node to be parsed

# Returns

[`true`] if the node was successfully parsed, [`false`] otherwise.
<!-- trait ScriptableExt::fn set_custom_property -->
Overrides the common properties setting. The underlying virtual
function should be used when implementing custom properties.
## `script`
the `ClutterScript` creating the scriptable instance
## `name`
the name of the property
## `value`
the value of the property
<!-- struct Stage -->
The [`Stage`][crate::Stage] structure contains only private data
and should be accessed using the provided API

# Implements

[`StageExt`][trait@crate::prelude::StageExt], [`ActorExt`][trait@crate::prelude::ActorExt], [`AnimatableExt`][trait@crate::prelude::AnimatableExt], [`ContainerExt`][trait@crate::prelude::ContainerExt], [`ScriptableExt`][trait@crate::prelude::ScriptableExt]
<!-- trait StageExt::fn actor_at_pos -->
Checks the scene at the coordinates `x` and `y` and returns a pointer
to the [`Actor`][crate::Actor] at those coordinates. The result is the actor which
would be at the specified location on the next redraw, and is not
necessarily that which was there on the previous redraw. This allows the
function to perform chronologically correctly after any queued changes to
the scene, and even if nothing has been drawn.

By using `pick_mode` it is possible to control which actors will be
painted and thus available.
## `pick_mode`
how the scene graph should be painted
## `x`
X coordinate to check
## `y`
Y coordinate to check

# Returns

the actor at the specified coordinates,
 if any
<!-- trait StageExt::fn capture_final_size -->
Get the size of the framebuffer one must pass to
`clutter_stage_paint_to_buffer()` or `clutter_stage_paint_to_framebuffer()`
would be used with the same `rect`.
## `rect`
a `cairo_rectangle_int_t`

# Returns

[`true`] if the size has been retrieved, [`false`] otherwise.

## `out_width`
the final width

## `out_height`
the final height

## `out_scale`
the final scale factor
<!-- trait StageExt::fn device_actor -->
Retrieves the [`Actor`][crate::Actor] underneath the pointer or touch point
of `device` and `sequence`.
## `device`
a [`InputDevice`][crate::InputDevice]
## `sequence`
an optional `ClutterEventSequence`

# Returns

a pointer to the [`Actor`][crate::Actor] or [`None`]
<!-- trait StageExt::fn perspective -->
Retrieves the stage perspective.

# Returns


## `perspective`
return location for a
 `ClutterPerspective`
<!-- trait StageExt::fn paint_to_buffer -->
Take a snapshot of the stage to a provided buffer.
## `rect`
a `cairo_rectangle_int_t`
## `scale`
the scale
## `data`
a pointer to the data
## `stride`
stride of the image surface
## `format`
the pixel format
## `paint_flags`
the `ClutterPaintFlag`

# Returns

[`true`] is the buffer has been paint successfully, [`false`] otherwise.
<!-- trait StageExt::fn read_pixels -->
Makes a screenshot of the stage in RGBA 8bit data, returns a
linear buffer with `width` * 4 as rowstride.

The alpha data contained in the returned buffer is driver-dependent,
and not guaranteed to hold any sensible value.
## `x`
x coordinate of the first pixel that is read from stage
## `y`
y coordinate of the first pixel that is read from stage
## `width`
Width dimension of pixels to be read, or -1 for the
 entire stage width
## `height`
Height dimension of pixels to be read, or -1 for the
 entire stage height

# Returns

a pointer to newly allocated memory with the buffer
 or [`None`] if the read failed. Use `g_free()` on the returned data
 to release the resources it has allocated.
<!-- trait StageExt::fn connect_after_paint -->
The ::after-paint signal is emitted after the stage is painted,
but before the results are displayed on the screen.
## `view`
a `ClutterStageView`
<!-- trait StageExt::fn connect_after_update -->
## `view`
a `ClutterStageView`
<!-- trait StageExt::fn connect_before_paint -->
The ::before-paint signal is emitted before the stage is painted.
## `view`
a `ClutterStageView`
<!-- trait StageExt::fn connect_before_update -->
## `view`
a `ClutterStageView`
<!-- trait StageExt::fn connect_paint_view -->
The ::paint-view signal is emitted before a `ClutterStageView` is being
painted.

The view is painted in the default handler. Hence, if you want to perform
some action after the view is painted, like reading the contents of the
framebuffer, use `g_signal_connect_after()` or pass `G_CONNECT_AFTER`.
## `view`
a `ClutterStageView`
## `redraw_clip`
a `cairo_region_t` with the redraw clip
<!-- trait StageExt::fn connect_presented -->
Signals that the [`Stage`][crate::Stage] was presented on the screen to the user.
## `view`
the `ClutterStageView` presented
## `frame_info`
a `ClutterFrameInfo`
<!-- enum StaticColor::variant White -->
White color (ffffffff)
<!-- enum StaticColor::variant Black -->
Black color (000000ff)
<!-- enum StaticColor::variant Red -->
Red color (ff0000ff)
<!-- enum StaticColor::variant DarkRed -->
Dark red color (800000ff)
<!-- enum StaticColor::variant Green -->
Green color (00ff00ff)
<!-- enum StaticColor::variant DarkGreen -->
Dark green color (008000ff)
<!-- enum StaticColor::variant Blue -->
Blue color (0000ffff)
<!-- enum StaticColor::variant DarkBlue -->
Dark blue color (000080ff)
<!-- enum StaticColor::variant Cyan -->
Cyan color (00ffffff)
<!-- enum StaticColor::variant DarkCyan -->
Dark cyan color (008080ff)
<!-- enum StaticColor::variant Magenta -->
Magenta color (ff00ffff)
<!-- enum StaticColor::variant DarkMagenta -->
Dark magenta color (800080ff)
<!-- enum StaticColor::variant Yellow -->
Yellow color (ffff00ff)
<!-- enum StaticColor::variant DarkYellow -->
Dark yellow color (808000ff)
<!-- enum StaticColor::variant Gray -->
Gray color (a0a0a4ff)
<!-- enum StaticColor::variant DarkGray -->
Dark Gray color (808080ff)
<!-- enum StaticColor::variant LightGray -->
Light gray color (c0c0c0ff)
<!-- enum StaticColor::variant Butter -->
Butter color (edd400ff)
<!-- enum StaticColor::variant ButterLight -->
Light butter color (fce94fff)
<!-- enum StaticColor::variant ButterDark -->
Dark butter color (c4a000ff)
<!-- enum StaticColor::variant Orange -->
Orange color (f57900ff)
<!-- enum StaticColor::variant OrangeLight -->
Light orange color (fcaf3fff)
<!-- enum StaticColor::variant OrangeDark -->
Dark orange color (ce5c00ff)
<!-- enum StaticColor::variant Chocolate -->
Chocolate color (c17d11ff)
<!-- enum StaticColor::variant ChocolateLight -->
Light chocolate color (e9b96eff)
<!-- enum StaticColor::variant ChocolateDark -->
Dark chocolate color (8f5902ff)
<!-- enum StaticColor::variant Chameleon -->
Chameleon color (73d216ff)
<!-- enum StaticColor::variant ChameleonLight -->
Light chameleon color (8ae234ff)
<!-- enum StaticColor::variant ChameleonDark -->
Dark chameleon color (4e9a06ff)
<!-- enum StaticColor::variant SkyBlue -->
Sky color (3465a4ff)
<!-- enum StaticColor::variant SkyBlueLight -->
Light sky color (729fcfff)
<!-- enum StaticColor::variant SkyBlueDark -->
Dark sky color (204a87ff)
<!-- enum StaticColor::variant Plum -->
Plum color (75507bff)
<!-- enum StaticColor::variant PlumLight -->
Light plum color (ad7fa8ff)
<!-- enum StaticColor::variant PlumDark -->
Dark plum color (5c3566ff)
<!-- enum StaticColor::variant ScarletRed -->
Scarlet red color (cc0000ff)
<!-- enum StaticColor::variant ScarletRedLight -->
Light scarlet red color (ef2929ff)
<!-- enum StaticColor::variant ScarletRedDark -->
Dark scarlet red color (a40000ff)
<!-- enum StaticColor::variant Aluminium1 -->
Aluminium, first variant (eeeeecff)
<!-- enum StaticColor::variant Aluminium2 -->
Aluminium, second variant (d3d7cfff)
<!-- enum StaticColor::variant Aluminium3 -->
Aluminium, third variant (babdb6ff)
<!-- enum StaticColor::variant Aluminium4 -->
Aluminium, fourth variant (888a85ff)
<!-- enum StaticColor::variant Aluminium5 -->
Aluminium, fifth variant (555753ff)
<!-- enum StaticColor::variant Aluminium6 -->
Aluminium, sixth variant (2e3436ff)
<!-- enum StaticColor::variant Transparent -->
Transparent color (00000000)
<!-- struct Text -->
The [`Text`][crate::Text] struct contains only private data.

# Implements

[`TextExt`][trait@crate::prelude::TextExt], [`ActorExt`][trait@crate::prelude::ActorExt], [`AnimatableExt`][trait@crate::prelude::AnimatableExt], [`ContainerExt`][trait@crate::prelude::ContainerExt], [`ScriptableExt`][trait@crate::prelude::ScriptableExt]
<!-- trait TextExt::fn attributes -->
Gets the attribute list that was set on the [`Text`][crate::Text] actor
`clutter_text_set_attributes()`, if any.

# Returns

the attribute list, or [`None`] if none was set. The
 returned value is owned by the [`Text`][crate::Text] and should not be unreferenced.
<!-- trait TextExt::fn cursor_rect -->
Retrieves the rectangle that contains the cursor.

The coordinates of the rectangle's origin are in actor-relative
coordinates.

# Returns


## `rect`
return location of a `ClutterRect`
<!-- trait TextExt::fn ellipsize -->
Returns the ellipsizing position of a [`Text`][crate::Text] actor, as
set by `clutter_text_set_ellipsize()`.

# Returns

`PangoEllipsizeMode`
<!-- trait TextExt::fn font_description -->
Retrieves the `PangoFontDescription` used by `self`

# Returns

a `PangoFontDescription`. The returned value is owned
 by the [`Text`][crate::Text] actor and it should not be modified or freed
<!-- trait TextExt::fn layout -->
Retrieves the current `PangoLayout` used by a [`Text`][crate::Text] actor.

# Returns

a `PangoLayout`. The returned object is owned by
 the [`Text`][crate::Text] actor and should not be modified or freed
<!-- trait TextExt::fn line_alignment -->
Retrieves the alignment of a [`Text`][crate::Text], as set by
`clutter_text_set_line_alignment()`.

# Returns

a `PangoAlignment`
<!-- trait TextExt::fn line_wrap_mode -->
Retrieves the line wrap mode used by the [`Text`][crate::Text] actor.

See clutter_text_set_line_wrap_mode ().

# Returns

the wrap mode used by the [`Text`][crate::Text]
<!-- trait TextExt::fn set_attributes -->
Sets the attributes list that are going to be applied to the
[`Text`][crate::Text] contents.

The [`Text`][crate::Text] actor will take a reference on the `PangoAttrList`
passed to this function.
## `attrs`
a `PangoAttrList` or [`None`] to unset the attributes
<!-- trait TextExt::fn set_ellipsize -->
Sets the mode used to ellipsize (add an ellipsis: "...") to the
text if there is not enough space to render the entire contents
of a [`Text`][crate::Text] actor
## `mode`
a `PangoEllipsizeMode`
<!-- trait TextExt::fn set_font_description -->
Sets `font_desc` as the font description for a [`Text`][crate::Text]

The `PangoFontDescription` is copied by the [`Text`][crate::Text] actor
so you can safely call `pango_font_description_free()` on it after
calling this function.
## `font_desc`
a `PangoFontDescription`
<!-- trait TextExt::fn set_line_alignment -->
Sets the way that the lines of a wrapped label are aligned with
respect to each other. This does not affect the overall alignment
of the label within its allocated or specified width.

To align a [`Text`][crate::Text] actor you should add it to a container
that supports alignment, or use the anchor point.
## `alignment`
A `PangoAlignment`
<!-- trait TextExt::fn set_line_wrap_mode -->
If line wrapping is enabled (see [`set_line_wrap()`][Self::set_line_wrap()]) this
function controls how the line wrapping is performed. The default is
`PANGO_WRAP_WORD` which means wrap on word boundaries.
## `wrap_mode`
the line wrapping mode
<!-- trait TextExt::fn set_preedit_string -->
Sets, or unsets, the pre-edit string. This function is useful
for input methods to display a string (with eventual specific
Pango attributes) before it is entered inside the [`Text`][crate::Text]
buffer.

The preedit string and attributes are ignored if the [`Text`][crate::Text]
actor is not editable.

This function should not be used by applications
## `preedit_str`
the pre-edit string, or [`None`] to unset it
## `preedit_attrs`
the pre-edit string attributes
## `cursor_pos`
the cursor position for the pre-edit string
<!-- trait TextExt::fn connect_cursor_event -->
The ::cursor-event signal is emitted whenever the cursor position
changes inside a [`Text`][crate::Text] actor. Inside `rect` it is stored
the current position and size of the cursor, relative to the actor
itself.

# Deprecated since 1.16

Use the `signal::Text::cursor-changed` signal instead
## `rect`
the coordinates of the cursor
<!-- trait TextExt::fn connect_insert_text -->
This signal is emitted when text is inserted into the actor by
the user. It is emitted before `self_` text changes.
## `new_text`
the new text to insert
## `new_text_length`
the length of the new text, in bytes, or -1 if
 new_text is nul-terminated
## `position`
the position, in characters, at which to insert the
 new text. this is an in-out parameter. After the signal
 emission is finished, it should point after the newly
 inserted text.
<!-- trait TextExt::fn activatable -->
Toggles whether return invokes the activate signal or not.
<!-- trait TextExt::fn cursor_visible -->
Whether the input cursor is visible or not.

The cursor will only be visible if this property and either
the `property::Text::editable` or the `property::Text::selectable` properties
are set to [`true`].
<!-- trait TextExt::fn editable -->
Whether key events delivered to the actor causes editing.
<!-- trait TextExt::fn justify -->
Whether the contents of the [`Text`][crate::Text] should be justified
on both margins.
<!-- trait TextExt::fn line_wrap -->
Whether to wrap the lines of `property::Text::text` if the contents
exceed the available allocation. The wrapping strategy is
controlled by the `property::Text::line-wrap-mode` property.
<!-- trait TextExt::fn selectable -->
Whether it is possible to select text, either using the pointer
or the keyboard.

This property depends on the `property::Actor::reactive` property being
set to [`true`].
<!-- trait TextExt::fn single_line_mode -->
Whether the [`Text`][crate::Text] actor should be in single line mode
or not. A single line [`Text`][crate::Text] actor will only contain a
single line of text, scrolling it in case its length is bigger
than the allocated size.

Setting this property will also set the `property::Text::activatable`
property as a side-effect.

The `property::Text::single-line-mode` property is used only if the
`property::Text::editable` property is set to [`true`].
<!-- trait TextExt::fn use_markup -->
Whether the text includes Pango markup.

For more information about the Pango markup format, see
`pango_layout_set_markup()` in the Pango documentation.

It is not possible to round-trip this property between
[`true`] and [`false`]. Once a string with markup has been set on
a [`Text`][crate::Text] actor with :use-markup set to [`true`], the markup
is stripped from the string.
<!-- struct TextBuffer -->
The [`TextBuffer`][crate::TextBuffer] structure contains private
data and it should only be accessed using the provided API.

# Implements

[`TextBufferExt`][trait@crate::prelude::TextBufferExt]
<!-- impl TextBuffer::fn new_with_text -->
Create a new ClutterTextBuffer object with some text.
## `text`
initial buffer text
## `text_len`
initial buffer text length, or -1 for null-terminated.

# Returns

A new ClutterTextBuffer object.
