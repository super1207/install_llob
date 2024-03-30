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

## License

MIT