

pub fn run(target:&str)->&str{
    let info:Vec<&str>=target.split("*").collect();
    if info.len()==2{
        match info[0] {
            "emoji"=>{
                match info[1] {
                    "love"=>{
                        return "(｡･ω･｡)ﾉ♡";
                    },
                    "fear"=>{
                        return "°.°·(((p(≧□≦)q)))·°.°";
                    }
                    "angry"=>{
                        return "（╬￣皿￣）";
                    }
                    "go"=>{
                        return "<(￣︶￣)↗[GO!]";
                    }
                    "hi"=>{
                        return "Hi~ o(*￣▽￣*)ブ";
                    }
                    "haha"=>{
                        return "○( ＾皿＾)っHAHAHA…";
                    }
                    "welcome"=>{
                        return "( ＾∀＾）／欢迎＼( ＾∀＾）";
                    }
                    "oh_yeah"=>{
                        return "ε(*′･∀･｀)зﾞ";
                    }
                    "0.9?"=>{
                        return "╰(￣▽￣)╭";
                    }
                    &_ => {}
                }
            }
            &_ => {}
        }
    }
    target
}