use std::fs;

pub fn run(args:Config)-> Result<(),&'static str>{
    let contents = fs::read_to_string(args.filename);
    match contents {
        Ok(contents) => {
            for i in search(&args.query.to_lowercase(), &contents){
                println!("{:?}",i)
            }
        },
        Err(err) => eprintln!("{:?}",err)
    }

    Ok(())
}
 
pub struct Config<'a>{
    pub filename: &'a str,
    pub query: &'a str
}

impl<'a> Config<'a> {
    pub fn new(args:&Vec<String>)->Result<Config,&'static str> {    
        if args.len() < 3 {
            return Err("expected 2 arguments. First file location and Second word to search for")
        }
        Ok(Config{
            filename:&args[1],
            query:&args[2]
        })
    } 
}


pub fn search<'a>(query:&str,contents:&'a str) -> Vec<&'a str>{
    let mut  result = Vec::new();
    for line in contents.lines(){
        if (line.to_lowercase()).contains(query){
            result.push(line);
        }
    }
    result
}    
   

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query ="Fly";
        let contents = "Fly me to the moon\nLet me play among the stars\nAnd let me see what spring is like on\nA-Jupiter and Mars\n\nIn other words\nHold my hand\nIn other words\nBaby, kiss me\n\nFill my heart with song\nAnd let me sing forever more\nYou are all I long for\nAll I worship and adore\n\nIn other words\nPlease, be true\nIn other words\nI love you\n\nFill my heart with song\nLet me sing forever more\nYou are all I long for\nAll I worship and adore\n\nIn other words\nPlease, be true\nIn other words\nIn other words\nI love You\n\n";
        assert_eq!(vec!["Fly me to the moon"],search(query,contents));
    }
}   