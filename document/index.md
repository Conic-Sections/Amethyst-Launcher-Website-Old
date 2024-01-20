# 简介

> **提示：**
> 
> 请善用浏览器的`Ctrl + F`查找功能
> 
> 如果你发现某些文档内容难以理解或与事实不符，请及时联系管理员
> 
> 
> 如果你需要报告问题，请前往[GitHub Issues](https://github.com/Broken-Deer/magical-launcher/issues)报告问题

## 什么是 Hedwig 启动器?

Hedwig 是 Minecraft 启动器，用于帮助用户下载、启动 Minecraft，同时提供了许多有用的工具管理游戏数据。
Hedwig 基于 tauri 构建，并提供了一套安全的插件接口，方便用户自己向启动器添加功能并分享给其他用户，同时确保安装者安全。

下面是一个最基本的示例:

<!-- todo：演示视频 -->

上面的示例展示了 Hedwig 的几个核心功能:

- **安装游戏与启动**: 可以安装游戏或者启动游戏。
- **插件系统**: Hedwig 可以通过插件为启动器添加各种功能。

Hedwig 还具有许多有用的特性：

- 可以快速安装 `Minecraft`, `Forge`, `Fabric`, `Optifine`, `Quilt`, 会自动选择速度最快的镜像服务器安装游戏
- **跨平台**，使用rust编写的启动器，能够在 Windows, MacOS 和 GNU/Linux 上运行，~~甚至可以在手机上运行~~
- **宇宙无敌的下载速度**，在 Linux 内核的系统上安装游戏甚至只需要15秒
- **实例管理**，支持分组等功能，轻松管理多个实例
- **模组下载**，你可以直接在启动器内下载 Curseforge, Modrinth, FTB 上的模组
- **与 PCL2, HMCL, Bakaxl 联机**
- **支持多种账号系统**，内置支持 Microsoft 和 Mojang Yggdrasil API。内置支持 LittleSkin，您也可以自行添加新的第三方验证服务！
- **节约存储空间**，在多个实例中启用的相同材质包只存储一次
- **自动测试导致无法启动的模组**，可以自动测试哪些模组会在游戏启动时导致崩溃
- **自定义启动器外观**，启动器的每一寸角落都可以自定义
- **在启动器中修改存档的游戏规则**，你可以修改游戏规则、是否允许作弊等，还可以关闭极限模式
- **自定义下载源**，可以使用自建镜像源而无需自己编译启动器

## 开源协议

该程序在 GPL-3.0 开源协议下发布, 同时附有附加条款。当你使用启动器的源代码时，请遵循这些协议。

### 附加条款 (依据 GPL-3.0 开源协议第七条)

1. 当您分发该程序的修改版本时, 您必须以一种合理的方式修改该程序的名称或版本号, 以示其与原始版本不同. (依据 [GPL-3.0, 7(c)](https://github.com/Broken-Deer/magical-launcher/blob/master/LICENSE#L372-L374))

您需要自行在源码中查找所有与本程序名称相关的词语并替换为您自己程序的名称

2. 您不得移除该程序所显示的版权声明. (依据 [GPL-3.0, 7(b)](https://github.com/Broken-Deer/magical-launcher/blob/master/LICENSE#L368-L370))

3. 如果您和接收者签了合同性质的东西，并提供责任承诺，则授权人和作者不受此责任连带. (依据[GPL-3.0, 7(b)](https://github.com/Broken-Deer/magical-launcher/blob/master/LICENSE#L382-L386))
