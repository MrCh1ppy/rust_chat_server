use std::collections::HashMap;

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
                    &_ => {}
                }
            }
            &_ => {}
        }
    }
    target
}