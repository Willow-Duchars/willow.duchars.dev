pub mod icons {
    pub const MINIMIZE: &str = "public/minimize-window.svg";
    pub const MAXIMIZE: &str = "public/maximize-window.svg";
    pub const CLOSE: &str = "public/close-window.svg";
    pub const ABOUT: &str = "public/about.svg";
    pub const EXTERNAL: &str = "public/external.svg";
    pub const GITHUB: &str = "public/github-mark-white.svg";
    pub const LINKEDIN: &str = "public/LI-In-Bug.png";
    pub const MENU: &str = "public/menu.svg";
    pub const CRUCIBLE: &str = "public/crucible_icon.png";
}

pub mod external_links {
    pub const GITHUB: &str = "https://github.com/Willow-Duchars";
    pub const LINKEDIN: &str = "https://www.linkedin.com/in/willow-duchars/";
}

pub mod styling {
    pub const TASKBAR_BORDER_WIDTH: f64 = 1.0;
    pub const TASKBAR_TOTAL_PADDING: f64 = 4.0;
    pub const TASKBAR_HEIGHT: f64 = 45.0;
    pub const TASKBAR_TOTAL_HEIGHT: f64 =
        TASKBAR_BORDER_WIDTH + TASKBAR_TOTAL_PADDING + TASKBAR_HEIGHT;
}

pub mod alloy_calculator {
    use crate::{core::sprite_data::SpriteData, Dimensions};
    use leptos_use::core::Position;

    // =============== Sprite Data Constants ===============
    const COL_1: f64 = 0.0;
    const COL_2: f64 = -60.0;
    const COL_3: f64 = -120.0;
    const COL_4: f64 = -180.0;

    const ROW_1: f64 = 0.0;
    const ROW_2: f64 = -60.0;
    const ROW_3: f64 = -120.0;
    const ROW_4: f64 = -180.0;
    const ROW_5: f64 = -240.0;
    const ROW_6: f64 = -300.0;

    const SPRITESHEET: &str = "public/vs_alloy_calculator_spritesheet.png";
    const DEFAULT_SPRITE_SIZE: Dimensions = Dimensions { w: 60.0, h: 60.0 };

    // =============== Row 1 Sprites ===============
    pub const NUGGET_PENTLANDITE: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "pentlandite nugget",
        pos: Position { x: COL_1, y: ROW_1 },
        size: DEFAULT_SPRITE_SIZE,
    };
    pub const NUGGET_NATIVE_COPPER: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "native copper nugget",
        pos: Position { x: COL_2, y: ROW_1 },
        size: DEFAULT_SPRITE_SIZE,
    };
    pub const NUGGET_SPHALERITE: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "sphalerite nugget",
        pos: Position { x: COL_3, y: ROW_1 },
        size: DEFAULT_SPRITE_SIZE,
    };
    pub const NUGGET_NATIVE_SILVER: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "native silver nugget",
        pos: Position { x: COL_4, y: ROW_1 },
        size: DEFAULT_SPRITE_SIZE,
    };

    // =============== Row 2 Sprites ===============
    pub const NUGGET_CASSITERITE: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "cassiterite nugget",
        pos: Position { x: COL_1, y: ROW_2 },
        size: DEFAULT_SPRITE_SIZE,
    };
    pub const NUGGET_NATIVE_GOLD: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "native gold nugget",
        pos: Position { x: COL_2, y: ROW_2 },
        size: DEFAULT_SPRITE_SIZE,
    };
    pub const NUGGET_GALENA: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "galena nugget",
        pos: Position { x: COL_3, y: ROW_2 },
        size: DEFAULT_SPRITE_SIZE,
    };
    pub const NUGGET_BISMUTHINITE: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "bismuthinite nugget",
        pos: Position { x: COL_4, y: ROW_2 },
        size: DEFAULT_SPRITE_SIZE,
    };

    // =============== Row 3 Sprites ===============
    pub const INGOT_TIN_BRONZE: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "tin bronze ingot",
        pos: Position { x: COL_1, y: ROW_3 },
        size: DEFAULT_SPRITE_SIZE,
    };
    pub const INGOT_BISMUTH_BRONZE: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "bismuth bronze ingot",
        pos: Position { x: COL_2, y: ROW_3 },
        size: DEFAULT_SPRITE_SIZE,
    };
    pub const INGOT_BLACK_BRONZE: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "black bronze ingot",
        pos: Position { x: COL_3, y: ROW_3 },
        size: DEFAULT_SPRITE_SIZE,
    };
    pub const INGOT_BRASS: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "brass ingot",
        pos: Position { x: COL_4, y: ROW_3 },
        size: DEFAULT_SPRITE_SIZE,
    };

    // =============== Row 4 Sprites ===============
    pub const INGOT_MOLYBDOCHALKOS: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "molybdochalkos ingot",
        pos: Position { x: COL_1, y: ROW_4 },
        size: DEFAULT_SPRITE_SIZE,
    };
    pub const INGOT_LEAD_SOLDER: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "lead solder ingot",
        pos: Position { x: COL_2, y: ROW_4 },
        size: DEFAULT_SPRITE_SIZE,
    };
    pub const INGOT_SILVER_SOLDER: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "silver solder ingot",
        pos: Position { x: COL_3, y: ROW_4 },
        size: DEFAULT_SPRITE_SIZE,
    };
    pub const INGOT_ELECTRUM: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "electrum ingot",
        pos: Position { x: COL_4, y: ROW_4 },
        size: DEFAULT_SPRITE_SIZE,
    };

    // =============== Row 5 Sprites ===============
    pub const INGOT_CUPRONICKEL: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "cupronickel ingot",
        pos: Position { x: COL_1, y: ROW_5 },
        size: DEFAULT_SPRITE_SIZE,
    };
    pub const EMPTY_SLOT: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "empty slot",
        pos: Position { x: COL_2, y: ROW_5 },
        size: DEFAULT_SPRITE_SIZE,
    };
    pub const CRUCIBLE: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "crucible",
        pos: Position { x: COL_3, y: ROW_5 },
        size: DEFAULT_SPRITE_SIZE,
    };
    pub const CHARCOAL: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "charcoal",
        pos: Position { x: COL_4, y: ROW_5 },
        size: DEFAULT_SPRITE_SIZE,
    };

    // =============== Row 6 Sprites ===============
    pub const SLOT_HIGHLIGHT: SpriteData = SpriteData {
        src: SPRITESHEET,
        alt: "highlighted slot",
        pos: Position { x: COL_1, y: ROW_6 },
        size: Dimensions { w: 64.0, h: 64.0 },
    };
}
