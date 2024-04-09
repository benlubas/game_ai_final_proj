pub mod render {
    use rlbot_lib::rlbot::{Color, RenderMessage, RenderType, Vector3};

    use crate::utils::math::math::vec_new;

    pub fn line(from: &Vector3, to: &Vector3, color: Color) -> RenderMessage {
        RenderMessage {
            renderType: RenderType::DrawLine3D,
            color: Some(Box::new(color)),
            start: Some(from.clone()),
            end: Some(to.clone()),
            scaleX: 1,
            scaleY: 1,
            text: None,
            isFilled: true,
        }
    }

    pub fn text(pos: &Vector3, text: String, color: Color) -> RenderMessage {
        RenderMessage {
            renderType: RenderType::DrawLine3D,
            color: Some(Box::new(color)),
            start: Some(pos.clone()),
            end: None,
            scaleX: 1,
            scaleY: 1,
            text: Some(text),
            isFilled: true,
        }
    }

    pub fn cross(pos: &Vector3, size: f32, color: Color) -> Vec<RenderMessage> {
        vec![
            RenderMessage {
                renderType: RenderType::DrawLine3D,
                color: Some(Box::new(color.clone())),
                start: Some(vec_new(pos.x - size / 2., pos.y - size / 2., pos.z)),
                end: Some(vec_new(pos.x + size / 2., pos.y + size / 2., pos.z)),
                scaleX: 1,
                scaleY: 1,
                text: None,
                isFilled: true,
            },
            RenderMessage {
                renderType: RenderType::DrawLine3D,
                color: Some(Box::new(color)),
                start: Some(vec_new(pos.x - size / 2., pos.y + size / 2., pos.z)),
                end: Some(vec_new(pos.x + size / 2., pos.y - size / 2., pos.z)),
                scaleX: 1,
                scaleY: 1,
                text: None,
                isFilled: true,
            },
        ]
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
