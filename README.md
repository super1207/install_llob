## Introduction

在windows上全自动安装 [LiteLoaderQQNT](https://github.com/LiteLoaderQQNT/LiteLoaderQQNT) 和 [LLOnebOT](https://github.com/LLOneBot/LLOneBot)。

Fully automatic installation of LLOneboT and LiteLoaderQQNT on Windows.

## Usage

双击exe是你需要的全部，如果遇到网络问题，就多尝试几次。

Double clicking the exe is all you need, just try a few more times if you have network problems.

##  Installation Location

你可以通过编写配置文件`llob_install.json`来指定`QQ.exe`的目录，如：
```json
{
    "qq_exe_path":"D:\\NTQQ\\QQ.exe"
}
```

你也可以将`llob_install.exe`拖入`QQ.exe`所在目录再运行。

**否则，将自动从Windows注册表中读取`QQ.exe`的安装路径。**

优先级：配置文件 > llob_install.exe 目录 > 注册表

You can specify the directory of `QQ.exe` by writing a configuration file `llob_install.json`, like this:
```json
{
    "qq_exe_path":"D:\\NTQQ\\QQ.exe"
}
```

Alternatively, you can drag `llob_install.exe` into the directory where `QQ.exe` is located and then run it.

**Otherwise, the installation path of `QQ.exe` will be automatically read from the Windows registry.**

Priority: Configuration file > llob_install.exe directory > Registry

## Thanks

[LiteLoaderQQNT](https://github.com/LiteLoaderQQNT/LiteLoaderQQNT)

[LLOnebOT](https://github.com/LLOneBot/LLOneBot)

[QQNTFileVerifyPatch](https://github.com/LiteLoaderQQNT/QQNTFileVerifyPatch)

[LiteLoaderQQNT_Install](https://github.com/Mzdyl/LiteLoaderQQNT_Install)

## License

MIT

## 其它注意事项
1. 确保您已经安装最新版本的NTQQ，您应该在[QQ官网](https://im.qq.com/pcqq/index.shtml)下载的最新版本的NTQQ，并且通过双击QQ安装包来安装QQ。
2. 确保使用install_llob.exe之前，QQ处于关闭状态。
3. 如果QQ自动更新了，您可以重新运行install_llob.exe来安装llonebot，您的配置信息不会丢失。
4. 如果您仍然对安装过程有所疑惑，可以看看安装视频：[视频链接](https://files.catbox.moe/psdz7v.mp4)
