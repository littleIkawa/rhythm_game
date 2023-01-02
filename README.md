# リズムゲーム

## 概要

RustのエンジンBevy (0.9.1)によるリズムゲーム.
譜面の配置パターンを取ると演出とともに加点されるのが特徴.
また, 譜面エディット機能もある.
yamlファイルで譜面を記述できる.

## 動作環境

開発はM1 MacBook Air, macOS Big Sur 11.6.
BevyはクロスプラットフォームなのでWindows等でもビルド可能.

## 操作方法

1. 起動直後ホーム画面に移る（ローディング画面から遷移しない場合なにかおかしい）.
1. 基本的に矢印キーでカーソル操作をする.
1. Startで曲選択画面, Exitで終了する.
1. 選曲画面ではDキーで難易度（ルール）を変更, 矢印キー左右でカードを選択し, Zキーで決定する. また, Xキーでホームに戻る. また, Sキーを押すとスピード調整ができる（上下キーで0.1刻みで調整）.
1. ルールは三種類ある. Normal：特になし. Expert：パターン取得評価が発生する. Master：アドリブノーツが追加され、自分でパターンを構築できる.
1. ロードが終わると曲が始まり譜面が流れてくる.
1. ゲーム中、レーンが4つある. これを0,1,2,3とする.
1. D, C, Sキーでレーン0, F, V, Gで1, J, N, Hで2, K, M, Lで3番のレーンを叩ける. 叩いたときに0, 3レーンは赤, 1, 2レーンは緑色に光る.
1. タイミングよく流れてくるノーツをキャッチする. Perfect, Perfect（ズレあり）, Ok, Missの4段階.
1. ノーツには通常ノーツ（青色）とロングノーツ（白色）がある. 通常ノーツは叩いたタイミングのみで評価され, ロングノーツは叩いたあとボタンを押し続けると加点がつく. また離すタイミングも評価される.
1. ExpertまたはMasterルールでは, 特定の配置（例えばトリルを3ノーツ以上続けるなど）を取ると演出が出る（現状スコアには組み込まれないが追加する予定）.
1. 曲が完全に終了してから2秒経つとリザルトが表示される.
1. パターン評価がなされていた場合は取得したパターンの一覧が表示される. リストは矢印上下キーでスクロールできる.
1. リザルト画面でZキーまたはReturnキーを押すと曲選択画面に戻る.

## 起動方法

### ソースをビルドしてデバッグ用で起動する

1. （リポジトリから取得する場合）`git clone`する
1. `cargo run`で起動する.

### ビルド済みのバイナリを起動する

1. `rhythm_2`と同じ階層に`assets`以下`fonts`, `songs`等があるか確認する.
1. シェルから`rhythm_2`があるディレクトリに移動して実行する（Finder等から起動すると相対パス指定の関係でエラー落ちしてしまう. 修正予定）

### デバッグ時

デフォルトではデバッグモードになっている.
`cargo run --features bevy/dynamic`とすると, デバッグ用のシステムも含めてビルドされる.
デバッグモードで起動時はローディング画面にその旨が表示される.
これを解除するには`Cargo.toml`の`[features]`を編集するか, `cargo run`実行時に`--no-default-features`オプションを付けて実行する.

なお, デバッグプラグインはデバッグモードでのみモジュールとして組み入れられるため, このfeatureを外してビルドすればデバッグ用コマンドがバイナリに残ることはない.

機能例：

- プレイ中Eキーを押しながらR：予定されている残りのノーツを全て消去して即座にリザルトに移行できる.
- プレイ中Eキーを押しながらB：選曲メニューに戻る.
- プレイ中Oキー：（予めデバッグコード内で登録しておいた）レセプタの動きを見ることができる.
- （Bevy備え付けの機能）Escキー：ウィンドウを閉じる.

### エディタモード

- 選曲画面でEキーを押しながらZキーで決定すると, エディタモードで選択される. ここで自分の入力によって譜面情報をエクスポートできる.
- `all_song_data.yaml`に曲一覧情報が記述されているが, ここで`edit_freeze: false`を設定していないものは選択できない（アラートが出る）.
- エディタモードでは（通常モードとは異なり）常に小節番号や拍が更新されており, 曲開始以降に鍵を押すとそのタイミングが記録される.
- Eキーを押しながらQキーで終了する（自動では終了しないので注意）.
- 終了すると保存するか尋ねられる画面に移る. E+Sでセーブ, E+Dで破棄する（画面に説明が出る）. セーブした場合既存のノーツ情報とマージされた譜面データファイルがCargoプロジェクトの直下に吐き出される（不意に上書きしないための措置）.
- 現状ロングノーツをエディタで作ることはできない.
