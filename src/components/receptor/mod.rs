mod denim;
mod double_tap;
mod full_sync;
mod step_left;
mod step_right;
mod step_trill;
mod trill;

/// レセプタ構造体を全部読み込むための公開モジュール
pub mod prelude {
    pub use super::{
        double_tap::DoubleTapReceptor, full_sync::FullSyncReceptor, step_left::StepLeftReceptor,
        step_right::StepRightReceptor, trill::TrillReceptor,
    };
}

use bevy::prelude::*;

use crate::events::CatchNoteEvent;

/// ノーツの並びパターン
#[derive(Clone, Copy, Debug)]
pub enum NotesPattern {
    Denim,
    /// 同時押し
    FullSync,
    /// 左上がり階段
    StepLeft,
    StepRight,
    /// 縦連2
    DoubleTap,
    /// トリル. 続いた長さを持つ
    Trill(u32),
    /// 連続縦連（猶予長め）. トリルと同じところで判定
    MultipleTap(u32),
    /// 3列トリル.
    StepTrill(u32),
}
impl std::fmt::Display for NotesPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Trill(length) => {
                write!(f, "Trill x {}", length)
            }
            Self::MultipleTap(length) => {
                write!(f, "MultiTap x {}", length)
            }
            _ => {
                write!(f, "{:?}", self)
            }
        }
    }
}
impl NotesPattern {
    pub fn to_score(self) -> u32 {
        match self {
            NotesPattern::Denim => 2,
            _ => 1,
        }
    }
}

/// パターン受容体の機能を与えるトレイト.
/// 様々なノーツの配置パターンをキャッチできるようにするために機能を一般化する.
pub trait PatternReceptor: Default + Component {
    /// 初期化を行う. デフォルト状態に戻す実装がされているが, 何をもって初期化とするかそれぞれが実装することもできる.
    fn init(&mut self) {
        *self = Self::default();
    }

    /// 初期化されているかどうかを表す.
    fn is_init(&self) -> bool;

    /// 毎フレーム呼ばれる. 経過時刻等でリセットを行うか決める
    fn init_or_defer(&mut self, current_time: f64, bpm: f32);

    /// ノーツを入力し状態を更新する.
    /// 適宜リセット等も行えるが, init_or_deferのあとに呼ばれるためにあまり考えなくて大丈夫.
    fn input(&mut self, note_ev: &CatchNoteEvent);

    /// 加点パターンの条件を満たしたかどうかを調べ, 満たしていたら対応するパターン列挙子を返す.
    fn achieved(&self) -> Option<NotesPattern>;

    /// 入力可能かどうかを返す.
    fn is_available(&self) -> bool;
}
