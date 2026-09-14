## Getting Mouse Inputs in Bevy

Bevy provides several ways to handle mouse inputs, primarily through input events and the `Input<MouseButton>` resource.

### 1. Using `Input<MouseButton>` Resource

For checking the current state of mouse buttons (pressed, just pressed, just released), you can use the `Input<MouseButton>` resource. This is useful for continuous actions or checking states without reacting to every single click event.

```rust
use bevy::prelude::*;

fn mouse_button_input(mouse_button_input: Res<Input<MouseButton>>) {
    // Check if the left mouse button is currently held down
    if mouse_button_input.pressed(MouseButton::Left) {
        println!("Left mouse button is held down!");
    }

    // Check if the right mouse button was just pressed this frame
    if mouse_button_input.just_pressed(MouseButton::Right) {
        println!("Right mouse button was just pressed!");
    }

    // Check if the middle mouse button was just released this frame
    if mouse_button_input.just_released(MouseButton::Middle) {
        println!("Middle mouse button was just released!");
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, mouse_button_input)
        .run();
}
```

### 2. Using Mouse Input Events

For more granular control over mouse movements, wheel scrolling, and specific button events (e.g., precise click detection), Bevy offers several event types. You can listen for these events using event readers.

#### `CursorMoved` Event

Fired whenever the mouse cursor moves within the window.

```rust
use bevy::prelude::*;
use bevy::window::CursorMoved;

fn cursor_event_system(mut cursor_moved_events: EventReader<CursorMoved>) {
    for event in cursor_moved_events.read() {
        println!("Cursor moved to: {:?}", event.position);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, cursor_event_system)
        .run();
}
```

#### `MouseWheel` Event

Fired when the mouse scroll wheel is used.

```rust
use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;

fn mouse_wheel_event_system(mut mouse_wheel_events: EventReader<MouseWheel>) {
    for event in mouse_wheel_events.read() {
        println!("Mouse wheel scrolled: {:?}", event.y);
        // event.x for horizontal scroll, event.y for vertical scroll
        // event.unit indicates whether the scroll is by line or pixel
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, mouse_wheel_event_system)
        .run();
}
```

#### `MouseButtonInput` Event

This event is fired when a mouse button changes state (pressed or released). While `Input<MouseButton>` is often simpler for state checking, this event provides the raw input change.

```rust
use bevy::prelude::*;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;

fn mouse_button_input_event_system(mut button_events: EventReader<MouseButtonInput>) {
    for event in button_events.read() {
        match event.state {
            ButtonState::Pressed => {
                println!("{:?} pressed", event.button);
            }
            ButtonState::Released => {
                println!("{:?} released", event.button);
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, mouse_button_input_event_system)
        .run();
}
```

### Integration into a Bevy Application

You typically add these systems to the `Update` schedule (or a custom schedule) within your `App`'s build method or a plugin's build method to ensure they run every frame.

For game development, `Input<MouseButton>` is generally preferred for actions that need to react to a held button (e.g., continuous movement, aiming), while events are great for single-trigger actions or UI interactions that require precise click detection or cursor position. Remember to clear event readers at the end of each frame, though Bevy's `EventReader` automatically handles this when iterating with `read()`.