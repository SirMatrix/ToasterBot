use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::model::id::ChannelId;

use std::process::Command;
use std::env;
use std::path::Path;
use std::thread;
use std::time::Duration;




const TOASTER: &'static str = "toaster";
const SVN_UPDATE: &'static str = "svn";
const MOVE_SVN: &'static str = "mv";
const PYTHON: &'static str = "python";
static mut SVN_BOOL: bool = false;
static mut TOASTER_BOOL: bool = false;
static mut FILE_SIGN: bool = false;
const UPDATE_CHANNEL: ChannelId = ChannelId(832725602728935454);



pub fn toast(){
    dotenv::dotenv().expect("Failed to load .env file");

    let webd =  env::var("WEBDIR").expect("Expected a web directory");
    let gamef = env::var("GAMEDIR").expect("Expected a directory for game files");
    let sign = env::var("SIGN").expect("Unable to load python script");
    let priv_key = env::var("PRIV_KEY").expect("Unable to use private key!");

    unsafe{TOASTER_BOOL = true};
    let mut toasting =Command::new(TOASTER);
    toasting.arg(&webd)
    .arg(&gamef);
if let Ok(mut child) = toasting.spawn() {
    child.wait().expect("Failure");
    println!("Child exited successfully");
    unsafe{TOASTER_BOOL = false};
}else {
    println!("never started");
}
unsafe{FILE_SIGN == true};
let mut py = Command::new(PYTHON);
py.arg(&sign)
.arg(&webd)
.arg(&priv_key);
if let Ok(mut child) = py.spawn(){
    child.wait().expect("Failed to run program");
    println!("Child exited successfully");
    unsafe{FILE_SIGN == false};
}else{
    println!("Never Started")
}
    //let id = toasting.id();
    //println!("ID for toaster is {}", id);
    

}
pub fn svnup(){
    dotenv::dotenv().expect("Failed to load .env file");
    let gamef = env::var("GAMEDIR").expect("Expected a directory for game files");
    let bindi = env::var("BIN_DIR").expect("Expected a bin directory");
    let homed = env::var("HOMEDIR").expect("Expected a home directory");

  unsafe{SVN_BOOL = true};
    let dir = Path::new(&gamef);
    env::set_current_dir(&dir).unwrap();
    println!("Ha ha im in {}!", dir.display());


Command::new(MOVE_SVN)
    .arg("../.svn")
    .arg(".")
    .spawn()
    .expect("Could not move svn folder");

let mut svnupdate = Command::new(SVN_UPDATE);
svnupdate.arg("update");
if let Ok(mut child) = svnupdate.spawn(){
    child.wait().expect("Failure");
    println!("SVN stopped correctly");
}else {
    println!("Failed to start");
}

let tobin = Path::new(&bindi);
env::set_current_dir(&tobin).unwrap();
println!("Switched to {}. Now deleting debug files.", tobin.display());

Command::new("rm")
.arg("client.so.dbg")
.arg("server.so.dbg")
.arg("client.pdb")
.arg("server.pdb")
.arg("GameUI.pdb")
.spawn()
.expect("Failed to remove debug files");
println!("Sleeping for 5 seconds.");

thread::sleep(Duration::from_secs(5));
env::set_current_dir(&dir).unwrap();
println!("Switched back to {}", dir.display());


Command::new(MOVE_SVN)
.arg(".svn/")
.arg("../")
.spawn()
.expect("Couldnt move .svn back");

println!("Returning to home directory");

let home = Path::new(&homed);
env::set_current_dir(&home).unwrap();
unsafe{SVN_BOOL = false};

}


#[command]
pub async fn update(ctx: &Context, msg: &Message)  -> CommandResult {
 
   if msg.channel_id != UPDATE_CHANNEL{
        msg.channel_id.say(&ctx.http, "You can only run this in #pusher-of-updates").await?;
    }else{
        msg.channel_id.say(&ctx.http, "Toaster has been ran!").await?;
        toast();
    }
    if msg.channel_id != UPDATE_CHANNEL{return Ok(());}else{
  
        if unsafe{TOASTER_BOOL == false && FILE_SIGN == false}{
            msg.channel_id.say(&ctx.http, "Toaster has finished running and the files are now signed! Go ahead and post your update!").await?;
        }   
    }
 
 
    Ok(())
    
}
#[command]
pub async fn svnu(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.channel_id != UPDATE_CHANNEL{
        msg.channel_id.say(&ctx.http, "You can only run this in #pusher-of-updates").await?;
    }else{
        msg.channel_id.say(&ctx.http, "SVN is currently being updated").await?;
        svnup();
    }
    if msg.channel_id != UPDATE_CHANNEL{return Ok(());}else{
        if unsafe{SVN_BOOL == false}{
            msg.channel_id.say(&ctx.http, "SVN is now up to date. Please run ~update to update toast the files.").await?;
        }
    }

    
    Ok(())
}