# 快速上手

## 安装 Amethyst Launcher

快速体验 Amethyst Launcher 的方法就是安装它。你可以到[这里](https://www.magicallauncher.org/)或者[GitHub releases](https://github.com/Broken-Deer/magical-launcher)下载它。

## 使用开发版本

如果你想尝试开发版本，请看以下步骤：

1. 安装 [Git](https://git-scm.com/) 和 [Node.js](https://nodejs.org)。
2. 使用以下命令克隆存储库：

```sh
git clone https://github.com/Broken-Deer/magical-launcher.git
cd magical-launcher
```

3. 安装依赖:

```sh
npm install -g yarn
yarn install
```

4. 构建:

```sh
yarn tauri build
```

完成后，请进入 `src-tauri/target/release/` 目录，然后你将看到构建好的启动器

