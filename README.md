# リズムゲーム

## 概要

RustのエンジンBevy (0.8.1)によるリズムゲーム.
譜面の配置パターンを取ると演出とともに加点されるのが特徴.
tomlファイルで譜面を記述できる.

## 動作環境

開発はM1 MacBook Air, macOS Big Sur 11.6.
BevyはクロスプラットフォームなのでWindows等でもビルド可能.

## 操作方法

1. 起動すると曲選択画面.
1. 矢印キー左右でカードを選択し, Zキーで決定する.
1. ロードが終わると曲が始まり譜面が流れてくる.
1. ゲーム中、レーンが4つある. これを1,2,3,4とする.
1. DまたはCキーでレーン1, FまたはVで2, JまたはNで3, KまたはMで4番のレーンを叩ける.
1. タイミングよく流れてくるノーツをキャッチする. Perfect, Perfect（ズレあり）, Ok, Missの4段階.
1. 例えばトリルを3ノーツ以上続けるなど, 特定の配置を取ると演出が出る（現状スコアには組み込まれないが追加する予定）.
1. 曲が完全に終了してから2秒経つとリザルトが表示される.
1. リザルト画面でZキーを押すと曲選択画面に戻る.
1. ゲーム中常にEscキーで終了する（デバッグ用機能）.

## 起動方法

### ソースをビルドしてデバッグ用で起動する

1. （リポジトリから取得する場合）`git clone`する
1. `cargo run`で起動する.

### ビルド済みのバイナリを起動する

1. `rhythm_2`と同じ階層に`assets`以下`fonts`, `songs`等があるか確認する.
1. シェルから`rhythm_2`があるディレクトリに移動して実行する（Finder等から起動すると相対パス指定の関係でエラー落ちしてしまう. 修正予定）
