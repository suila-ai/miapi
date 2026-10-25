//! 生成される呼び出しビルダーが使う、型状態のマーカーと future の型。
//!
//! エンドポイントの必須パラメータはそれぞれビルダーの状態タプルのスロットを 1 つ占める。
//! スロットは [`Unset`] で始まり、パラメータを渡すと [`Set`] になる。
//! 呼び出しを await できるのは、すべてのスロットが [`Set`] になったときだけである。

use std::pin::Pin;

/// マーカー: 必須パラメータは設定済みである。
#[derive(Debug, Clone, Copy)]
pub struct Set;

/// マーカー: 必須パラメータはまだ設定されていない。
#[derive(Debug, Clone, Copy)]
pub struct Unset;

/// 呼び出しビルダーを await したときの future。
pub type CallFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, crate::Error>> + Send + 'a>>;
