use std::{fs::{self}, path::PathBuf, process::Command, str::FromStr, sync::Arc};

use regex::Regex;
use reqwest::header::{HeaderName, HeaderValue};
use time::UtcOffset;
use ::time::format_description;

fn get_qq_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let hkcu = winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE);
    let qq_setting: winreg::RegKey = hkcu.open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\QQ")?;
    let qq_path:String = qq_setting.get_value("UninstallString")?;
    let q = PathBuf::from_str(&qq_path)?.parent().ok_or("can't find qq path")?.to_owned();
    Ok(q)
}

fn is_qq_run(qq_path:&PathBuf) -> Result<bool, Box<dyn std::error::Error>>  {
    let system = sysinfo::System::new_all();
    let process_name = "QQ.exe";
    if let Some(process) = system.processes_by_name(process_name).next() {
        let process_exe_path = process.exe().ok_or("can't get process exe path")?;
        let process_path = process_exe_path.parent().ok_or("can't get process parent path")?;
        if process_path == qq_path {
            return Ok(true)
        }
    } 
    Ok(false)
}

fn http_post(rt_ptr:Arc<tokio::runtime::Runtime>,url:&str,user_agent:Option<&str>) -> Vec<u8> {
    let bin = rt_ptr.block_on(async {
        let client = reqwest::Client::builder().danger_accept_invalid_certs(true).no_proxy().build().unwrap();
        let mut req = client.get(url).body(reqwest::Body::from(vec![])).build().unwrap();
        if let Some(ua) = user_agent {
            req.headers_mut().append(HeaderName::from_str("User-Agent").unwrap(), HeaderValue::from_str(ua).unwrap());
        }
        let ret = client.execute(req).await;
        if ret.is_err() {
            log::error!("Failed to download file{:?}",ret.err().unwrap());
            app_exit();
        }
        let ret = ret.unwrap();
        let bin = ret.bytes().await;
        if bin.is_err() {
            log::error!("Failed to download file{:?}",bin.err().unwrap());
            app_exit();
        }
        let bin = bin.unwrap();
        bin.to_vec()
    });
    bin
}

fn is_admin() -> Result<bool, Box<dyn std::error::Error>>  {
    let output = Command::new("net")
        .arg("session")
        .output()?;
    if output.status.success() {
        Ok(true)
    } else {
        Ok(false)
    }
}


fn init_log(){
    // 初始化日志
    let format = "[year]-[month]-[day] [hour]:[minute]:[second]";

    // 获得utc偏移
    let utc_offset;
    if let Ok(v) = UtcOffset::current_local_offset() {
        utc_offset = v;
    } else {
        // 中国是东八区，所以这里写8 hour
        utc_offset = UtcOffset::from_hms(8,0,0).unwrap();
    }

    tracing_subscriber::fmt()
    .with_timer(tracing_subscriber::fmt::time::OffsetTime::new(
        utc_offset,
        format_description::parse(format).unwrap(),
    ))
    .with_ansi(false)
    .with_max_level(tracing::Level::INFO)
    .init();
}

fn app_exit() -> !{
    loop {
        let time_struct = core::time::Duration::from_millis(500);
        std::thread::sleep(time_struct);
    }
}


fn is_x86_64(exe_data: &[u8]) -> Result<bool, Box<dyn std::error::Error>> {
    use goblin::Object;
    match Object::parse(exe_data)? {
        Object::PE(pe) => Ok(pe.is_64),
        _ => Err("File is not a Windows PE file.".into()),
    }
}

fn iswin32(qq_exe_path:&PathBuf) -> Result<bool, Box<dyn std::error::Error>>  {
    let content = std::fs::read(qq_exe_path)?;
    if is_x86_64(&content)? {
        return Ok(false);
    }
    Ok(true)
}

pub async fn github_proxy() -> Option<String> {
    let urls_to_test = ["https://mirror.ghproxy.com/", "https://github.moeyy.xyz/","https://github.moeyy.cn/",""];
    let (tx, mut rx) =  tokio::sync::mpsc::channel(urls_to_test.len() + 1);
    for url in urls_to_test {
        let tx = tx.clone();
        tokio::spawn(async move{
            let client = reqwest::Client::builder().danger_accept_invalid_certs(true).no_proxy().build().unwrap();
            let uri = reqwest::Url::from_str(&(url.to_owned() + 
				"https://raw.githubusercontent.com/super1207/redreply/master/res/version.txt")).unwrap();
            let req = client.get(uri).build().unwrap();
            if let Ok(ret) = client.execute(req).await {
                if ret.status() == reqwest::StatusCode::OK {
                    if let Ok(text) = ret.text().await {
                        let re = Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
                        if re.is_match(&text) {
                            let _err = tx.send(url).await;
                        } 
                    }
                }
            }; 
        });
    }
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        let _err = tx.send("timeout").await;
    });
    let ret = rx.recv().await;
    if let Some(r) = ret {
        if r != "timeout" {
            return Some(r.to_owned());
        }
    }
    None
}

fn extrat(from:&PathBuf,to:&PathBuf,flag:bool) -> Result<(),Box<dyn std::error::Error>>{
    let file = std::fs::File::open(from)?;

    let mut archive = zip::ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => {
                // write by chatgpt4
                let deal_path = path;
                let components: Vec<_> = deal_path.components().collect();
                if flag {
                    // println!("components:{components:?}");
                    if components.len() > 1 {
                        // 从第二个组件开始收集，直到倒数第二个（不包括最后一个组件）
                        let new_path = components[1..components.len()]
                            .iter()
                            .map(|c| c.as_os_str())
                            .collect::<PathBuf>();
                        to.join(new_path)
                    } else {
                        continue;
                        //return Err("Path is too short to remove the last component".into());
                    }
                }else{
                    let new_path = components[0..components.len()]
                        .iter()
                        .map(|c| c.as_os_str())
                        .collect::<PathBuf>();
                    to.join(new_path)
                }
                
            },
            None => continue,
        };

        
        {
            let comment = file.comment();
            if !comment.is_empty() {
                log::error!("File {i} comment: {comment}");
            }
        }

        if (*file.name()).ends_with('/') {
            // log::info!("File {} extracted to \"{}\"", i, outpath.display());
            std::fs::create_dir_all(&outpath)?;
            
            
        } else {
            // log::info!(
            //     "File {} extracted to \"{}\" ({} bytes)",
            //     i,
            //     outpath.display(),
            //     file.size()
            // );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
            }
        }
    }
    Ok(())
}


fn main(){
    if let Err(e) = mymain() {
        log::error!("{e:?}");
        app_exit();
    }
    app_exit();
}

fn mymain() -> Result<(), Box<dyn std::error::Error>>{


    let rt_ptr:Arc<tokio::runtime::Runtime> = Arc::new(tokio::runtime::Runtime::new().unwrap());

    init_log();

    log::info!("欢迎使用LLOB安装器0.0.1 by super1207");

    log::info!("正在检查是否拥有管理员权限...");
    if is_admin().unwrap() {
        log::info!("拥有管理员权限");
    } else {
        log::error!("没有管理员权限");
        app_exit();
    }

    log::info!("正在查询QQ安装位置...");
    let qq_path;
    if let Ok(qq_path_t) = get_qq_path() {
        qq_path = qq_path_t;
        log::info!("QQ安装位置: {:?}", qq_path);
    } else {
        log::error!("未找到QQ安装位置,请去安装QQ!：https://im.qq.com/pcqq/index.shtml");
        app_exit();
    }

    log::info!("正在检查QQ是否正在运行...");
    match is_qq_run(&qq_path) {
        Ok(is_run) => {
            if !is_run {
                log::info!("QQ未运行");
            }else{
                log::error!("QQ正在运行，请先结束QQ");
                app_exit();
            }
        },
        Err(err) => {
            log::error!("无法检查QQ是否正在运行:{err:?}");
            app_exit();
        }
    }

    log::info!("正在检查QQ位数...");
    let is_win32 = iswin32(&qq_path.join("QQ.exe"))?;
    if is_win32 {
        log::info!("您安装的是32位的QQ");
    }else{
        log::info!("您安装的是64位的QQ");
    }

    log::info!("正在获取github下载代理...");
    let git_proxy = rt_ptr.block_on(async {
        if let Some(proxy_t) = github_proxy().await {
            if proxy_t == "" {
                log::info!("无需使用代理即可连接github");
            }else{
                log::info!("使用代理: {:?}", proxy_t);
            }
            return proxy_t;
        } else {
            log::error!("无法获取github代理");
            app_exit();
        }
    });

    log::info!("正在获取最新QQNTFileVerifyPatch版本号...");
    let url = "https://api.github.com/repos/LiteLoaderQQNT/QQNTFileVerifyPatch/releases/latest";
    let bin = http_post(rt_ptr.clone(),url,Some("Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.72 Safari/537.36"));
    let version_json:serde_json::Value = serde_json::from_slice(&bin)?;
    let tag_name = version_json["tag_name"].as_str().ok_or("Failed to get tag_name")?;
    log::info!("最新QQNTFileVerifyPatch版本号:{tag_name}");

    log::info!("正在下载修补文件...");
    let patch_url;
    if is_win32 {
        patch_url = format!("{git_proxy}https://github.com/LiteLoaderQQNT/QQNTFileVerifyPatch/releases/download/{tag_name}/dbghelp_x86.dll");
    }else{
        patch_url = format!("{git_proxy}https://github.com/LiteLoaderQQNT/QQNTFileVerifyPatch/releases/download/{tag_name}/dbghelp_x64.dll");
    }
    let bin = http_post(rt_ptr.clone(),&patch_url,None);
    log::info!("修补文件下载完成");

    log::info!("正在修补...");
    fs::write(qq_path.join("dbghelp.dll"), bin)?;
    log::info!("修补完成");

    log::info!("正在下载LiteLoader项目...");
    let patch_url = format!("{git_proxy}https://github.com/LiteLoaderQQNT/LiteLoaderQQNT/archive/master.zip");
    let bin = http_post(rt_ptr.clone(),&patch_url,None);
    log::info!("下载完成");

    log::info!("正在解压...");
    let zip_path = qq_path.join("resources").join("app").join("LiteLoaderQQNT-main.zip");
    fs::write(&zip_path, bin)?;
    extrat(&zip_path, &zip_path.parent().ok_or("can't get parent")?.join("LiteLoaderQQNT-main"),true)?;
    log::info!("解压完成");

    let mut is_install = false;
    let index_file_path = qq_path.join("resources").join("app").join("app_launcher").join("index.js");
    if let Ok(index_content) = fs::read_to_string(&index_file_path) {
        if index_content.contains("LiteLoaderQQNT") {
            is_install = true;
        }
    }

    if is_install {
        log::info!("LiteLoaderQQNT已安装");
    }else {
        log::info!("正在安装LiteLoaderQQNT...");
        fs::write(index_file_path, "require(`../LiteLoaderQQNT-main`);\r\nrequire('./launcher.node').load('external_index', module);")?;
        log::info!("LiteLoaderQQNT安装完成");
    }

    log::info!("正在获取最新LLOB版本号...");
    let url = "https://api.github.com/repos/LLOneBot/LLOneBot/releases/latest";
    let bin = http_post(rt_ptr.clone(),url,Some("Mozilla/5.0 (Windows NT 6.1; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.72 Safari/537.36"));
    let version_json:serde_json::Value = serde_json::from_slice(&bin)?;
    let tag_name = version_json["tag_name"].as_str().ok_or("Failed to get tag_name")?;
    log::info!("最新LLOB版本号:{tag_name}");

    log::info!("正在下载LLOB项目...");

    let patch_url = format!("{git_proxy}https://github.com/LLOneBot/LLOneBot/releases/download/{tag_name}/LLOneBot.zip");
    let bin = http_post(rt_ptr.clone(),&patch_url,None);
    log::info!("下载完成");

    log::info!("正在安装LLOnebOT...");
    let zip_path = qq_path.join("resources").join("app").join("LiteLoaderQQNT-main").join("plugins").join(format!("LLOneBot{tag_name}.zip"));
    std::fs::create_dir_all(zip_path.parent().ok_or("can't get parent")?)?;
    fs::write(&zip_path, bin)?;
    extrat(&zip_path, &zip_path.parent().ok_or("can't get parent")?.join("LLOneBot"),false)?;
    log::info!("安装完成");


    let username = std::env::var("USERNAME")?;
    log::info!("正在授予当前用户QQ文件夹访问权限...");
    let mut binding = Command::new("icacls");
    let cm = binding
        .arg(format!("{}\\*",qq_path.to_string_lossy().to_string()))
        .arg("/t")
        .arg("/grant")
        .arg(format!("{username}:F"));
 
    let output = cm.output()?;
    if output.status.success() {
        log::info!("授予成功！！！！！！！！！享受快乐时光吧");
    } else {
        log::error!("授予失败:{:?}", String::from_utf8_lossy(&output.stderr));
        app_exit();
    }

    Ok(())
}
