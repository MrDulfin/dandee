use std::{env::args, fmt::Error};

use opener::open_browser;
use rand::Rng;
use tokio::task::spawn_blocking;

#[allow(clippy::single_match)]
fn main() -> Result<(), u16> {
    let mut args = args();
    if let Some(input) = args.nth(1) {

        match input.to_lowercase().as_str() {
            "--version" => version(),
            bun if bun == "--help" || bun == "help" => help(),
            "--errors" => {
                error();
            }
            "anime" =>  {
                println!("This feature is not finished!")

            },
            "define" => {
                if let Err(e) = define(args.next()) {
                    return Err(e);
                };
            },
            bun if (bun == "nanahira" || bun == "ななひら") => {

                if let Err(e) = nanahira(args.next()) {
                    return Err(e);
                };
            },

            _ => println!("🧸 Feature not supported! 🧸"),
        }
    }else {
        version();
        help()
    }
    Ok(())
}

fn version() {
    println!("🧸 dandee version 0.0.2 🧸")
}
fn help() {
    println!(r#"                                🧸 It's Dandee! 🧸
    syntax: dandee <command> --flags
 🧸 ==================================================================== 🧸

    Commands:

    nanahira - open a Youtube tab to one of my favourite songs by ななひら
    define - get the definition of a word... it doesn't work very well
    anime - not [yet] ready!
 🧸 ==================================================================== 🧸

    Flags:

    --version - show the version of this cli
    --help - show this wall of text, or options for another command
    --errors - list the error numbers and their meanings"#)
}

fn error() {
    println!(r#"🧸 The Errors in question:
    0: Missing arguments
    1: Problem opening link
    2: Song name provided does not match any available songs in the nanahira command options
    3: No definition could be found for provided word"#)
}
fn anime() -> Result<String, Error> {


    todo!()
}

fn define(word: Option<String>) -> Result<(), u16> {
    if let Some(word) = word {
        if word.as_str() == "--help" {
            println!("syntax: dandee define <word>");
            return Ok(());
        }
        match webster::dictionary(&word) {
            Some(def) => println!("{def}"),
            None => {
                println!("🧸 No definition found for {word}! 🧸");
                return Err(3)
            }
        }
    }else {
        println!("⚡🧸 You should give me a word to define... NOW!! 🧸⚡\nsyntax: dandee define <word>")
    }

    Ok(())
}

fn nanahira(val: Option<String>) -> Result<(), u16> {

    let num = if let Some(song) = val {

        match song.to_lowercase().as_str() {
            "--help" => {
                println!(r#"🧸 nanahira - open a Youtube tab to one of my favourite songs by ななひら 🧸
    syntax: dandee nanahira <option>

    options:
    <none> - random pick
    gravity - チョコレートグラビティ / Chocolate Gravity
    berry - Sweet Berry Love
    password - トゥインクル・パスワード / Twinkle Password
    cache - cache cache
    gemmarca - Gemmarca
    sho-pa - 🍻酔っパラパラパラパラダイス🍻
    nyan - Nyan loves you♡
    christmas - クリスマスなんて興味ないけど / But I'm not interested in Christmas though
    nanamega - 世界せいふく！ななmega王国
    origamical - オリガミカル・スウィートラヴ(Long ver.) / Origamical Sweet Love(Long ver.)
    soflan - 混乱少女♥そふらんちゃん!! (ノリニクシティ・アンリミテッド☆プログレッシヴ...)
    secret - ひみつのジュテーム♪
    cake - piece of cake
    midnight - ミッドナイト、スターライト / Midnight Starlight
    inori - Inori
    tokimeki - トキメキ×オンナノコ宣言
                            "#
                        );
                        return Ok(());
            },
            bun if (bun == "gravity" ||bun == "chocolate gravity") => 0,
            bun if (bun == "berry" || bun == "sweet berry love") => 1,
            bun if bun.contains("twinkle") || bun.contains("password") || bun == "twinkle-password" => 2,
            bun if bun == "cache" || bun == "cache-cache" => 3,
            "gemmarca" => 4,
            bun if bun == "sho-pa" || bun == "sake" => 5,
            bun if bun == "nyan" || bun == "nyan-loves-you" => 8,
            "christmas" => 9,
            bun if bun == "nanamega" || bun == "kingdom" || bun == "nanamega-kingdom" => 10,
            bun if bun == "origamical" || bun == "origamical-sweet-love" || bun == "origami" => 12,
            bun if bun.contains("soflan") => 13,
            "secret" => 16,
            bun if bun.contains("cake") => 17,
            bun if bun.contains("midnight") || bun.contains("starlight") => 18,
            "inori" => 19,
            "tokimeki" => 20,
            &_ => {
                println!("No such song supported [yet]!");
                return Err(2);
            },
        }

    }else {
        rand::thread_rng().gen_range(0..21)
    };

    let link: (&str, &str) = match num {
        0 => ("チョコレートグラビティ / Chocolate Gravity", "https://www.youtube.com/watch?v=6Pd_s_rsweQ"),
        1 => ("Sweet Berry Love", "https://youtu.be/jqNR6ANFgzA"),
        2 => ("トゥインクル・パスワード / Twinkle Password", "https://youtu.be/hHtdA5SofZc"),
        3 => ("cache cache", "https://youtu.be/UBVcsNc_VOE"),
        4 => ("Gemmarca", "https://youtu.be/jQEdk2HM7As"),
        5 => ("🍻酔っパラパラパラパラダイス🍻", "https://youtu.be/P-y0ZmaS2nE"),
        6 => ("喋蝶結び", "https://youtu.be/6qKPSd6dEjs"),
        7 => ("めっちゃ煽ってくるタイプの音ゲーボス曲ちゃんなんかに負けないが？...", "https://youtu.be/FEkKqt_ok8c"),
        8 => ("Nyan loves you♡", "https://youtu.be/7U35uu6n_1U"),
        9 => ("クリスマスなんて興味ないけど / But I'm not interested in Christmas though", "https://youtu.be/jFt4GIgtpBE"),
        10 => ("世界せいふく！ななmega王国", "https://youtu.be/i2-yPNtHDnY"),
        11 => ("真冬のシンデレラ", "https://youtu.be/lp3wSOLnnYE"),
        12 => ("オリガミカル・スウィートラヴ(Long ver.) / Origamical Sweet Love(Long ver.)", "https://youtu.be/dbgVSlwRfyw"),
        13 => ("混乱少女♥そふらんちゃん!! (ノリニクシティ・アンリミテッド☆プログレッシヴ・ポリリズミック・ナイトメア・ロングバージョン)\n
                    / Confusion Girl ♥ Soflan-chan!!(Nightmare Long Version)", "https://youtu.be/YdnOym-kbh4"),
        14 => ("迯避論", "https://youtu.be/o_m8bQmxGK0"),
        15 => ("ダニエルは女の子です！", "https://www.youtube.com/watch?v=BS5VmOFtsXA"),
        16 => ("ひみつのジュテーム♪", "https://www.youtube.com/watch?v=bcN2Tj1fPnY"),
        17 => ("piece of cake", "https://www.youtube.com/watch?v=NiB8QQ26cfI"),
        18 => ("ミッドナイト、スターライト / Midnight Starlight", "https://www.youtube.com/watch?v=5lUvv1NJyA8"),
        19 => ("Inori", "https://www.youtube.com/watch?v=yX-fxZ9DGi0"),
        20 => ("トキメキ×オンナノコ宣言", "https://www.youtube.com/watch?v=s5h386inK-Y"),
        21 => ("レンジで好吃☆電子調理器使用中華料理四千年歴史瞬間調理完了武闘的料理長☆", "https://www.youtube.com/watch?v=di2E67Ufweo"),
        _ => panic!()
    };

    println!("Now opening: {}...", link.0);
    if open_browser(link.1).is_err() {
        println!("couldn't open uri");
        return Err(1);
    };
    Ok(())
}