pub mod render {
    use rlbot_lib::rlbot::{Color, Vector3, RenderMessage, RenderType};

    pub fn line(from: Vector3, to: Vector3, color: Color) -> RenderMessage {
        RenderMessage {
            renderType: RenderType::DrawLine3D,
            color: Some(Box::new(color)),
            start: Some(from),
            end: Some(to),
            scaleX: 1,
            scaleY: 1,
            text: None,
            isFilled: true,
        }
    }

    pub fn text(pos: Vector3, text: String, color: Color) -> RenderMessage {
        RenderMessage {
            renderType: RenderType::DrawLine3D,
            color: Some(Box::new(color)),
            start: Some(pos),
            end: None,
            scaleX: 1,
            scaleY: 1,
            text: Some(text),
            isFilled: true,
        }
    }

    pub const RED: Color = Color {
        a: 255,
        r: 255,
        g: 0,
        b: 0,
    };
    pub const GREEN: Color = Color {
        a: 255,
        r: 0,
        g: 255,
        b: 0,
    };
    pub const BLUE: Color = Color {
        a: 255,
        r: 0,
        g: 0,
        b: 255,
    };
    pub const YELLOW: Color = Color {
        a: 255,
        r: 255,
        g: 255,
        b: 50,
    };
}
