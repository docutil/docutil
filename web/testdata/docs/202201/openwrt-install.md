# OpenWrt 安装流水账

**回忆版**

OpenWrt 是一个用于路由器的 Linux 发行版，适配了很多路由器，同时也支持一般的 x86 PC 机。我使用的正是 x86_64，用于我的矿渣 NAS 机器。

OpenWrt 主要是面向嵌入式设备开发的系统，占用资源比较少，而且自带一个不错的 Web 管理界面。对于我的 NAS 需求来说正好适合。毕竟 300 元的矿渣性能比树梅派好些之外，也不要求太多。

[TOC]

## NAS 方案

回顾一下矿渣机器的硬件情况：4核 CPU、4G 内存、4 个硬盘位，还有一个内置的 16GB SSD。

之前我是直接使用 PuppyLinux。PuppyLinux 属于 Live cd 性质的 Linux 裁减版本。安装比较容易。考虑到机器是矿渣的性能，不知道哪一天会坏。

组建 NAS 的目的就是保存手机的照片，所以只偶尔开机上传一下照片。硬盘虽然有 4 个位置，我只用两个 1TB 的硬盘作为核心存储，有一个 500G 放杂物。没有组 RAID，因为 RAID 挂了的话，恢复绝对是一个技术活。

我一般先上传文件到 A 盘，然后用 `rsync` 同步到的 B 盘中。上传文件通过 Samba 完成，理论上可以支持 Windows、macOS。 

保存数据，多硬盘独立储存，多存几份才是硬道理。数字时代保存数据是一个重要课题。

## 下载

推荐在官网 [https://openwrt.org/][openwrt-home] 下载镜像。国内可以在[清华的镜像][openwrt-mirror]下载。

我下载的是最新版本 [OpenWrt 21.02.1 的 EFI 镜像][openwrt-efi-img]

[openwrt-home]: https://openwrt.org/
[openwrt-mirror]: https://mirrors.tuna.tsinghua.edu.cn/openwrt/
[openwrt-efi-img]: https://mirrors.tuna.tsinghua.edu.cn/openwrt/releases/21.02.1/targets/x86/64/openwrt-21.02.1-x86-64-generic-ext4-combined-efi.img.gz

另准备一个 U 盘，下载 Puppy Linux 用于刷机。OpenWrt 是面向嵌入式的环境，需要上位机来安装、更新系统。

[puppy-linux-download]: https://puppylinux.com/index.html#download

windows 可以使用 [Refus](https://rufus.ie/zh/) 安装 Puppy Linux 到 U 盘。

## 安装

主要安装过程官网上已经有教程：https://openwrt.org/docs/guide-user/installation/openwrt_x86

将 NAS 从 Puppy Linux 的 U 盘启动，就可以开始安装 OpenWrt。

关键步骤：

```sh
$ gunzip openwrt-*.img.gz
$ lsblk # 查看已有的块设备，注意不要刷错硬盘
$ dd if=openwrt-21.02.0-x86-64-generic-ext4-combined.img bs=1M of=/dev/sdX # 在我的机器上是 /dev/sdb
```

`dd` 命令结束后，OpenWrt 就可以使用了。

按照官网上的说明，OpenWrt 的镜像默认为两个分区，只使用了 128MB 硬盘空间。如果需要使用硬盘的全部空间，需要手工扩容。

### Root 扩容

扩容我使用的 GParted。GParted 是 Puppy Linux 自带的分区操作软件。关键是 GParted 是基于 GUI 的，比较容易操作。

在 GParted 中打开 `/dev/sdb`，可以看到 /dev/sdb1 和 /dev/sdb2。扩容操作如下：

1. 选中 /dev/sdb2，右键选择 unmounted
2. 选中 /dev/sdb2，右键选择 resize
3. 在弹出的窗口中，上部分有一个控制条，拉到最右，使用全部可用的空间。
4. 保存 -> 应用

因为是 Live Linux 中进行操作的，扩容暂时没有发现文件丢失。

**重要** 扩容完成后需要更新 /dev/sdb1/boot/grub/grub.cfg 的中关于 ROOT PARTUUID 设置。

1. 查看新的 UUID：`lsblkl -n -o PARTUUID /dev/sdb2`
2. 编辑 `/dev/sdb1/boot/grub/grub.cfg` 将启动菜单中的 `root=PARTUUID=xxxx-xx-xxx-xxx-xxx` 替换为新的 PARTUUID。

扩容完成，重启机器进入 OpenWrt 环境。

## 系统设置

### 设置 root 密码

启动后，默认就会使用 root 账号登录系统。

```sh
$ passwd
```

按提示设置密码

### 设置网络

我 OpenWrt 只是用于提供文件共享服务，因此 OpenWrt 是放置到网络的路由器之下。需要设置 WAN 口为 DHCP 模式。我的机器只有一张网卡，因此被默认设置了 LAN 口。需要手工设置 WAN 口。

编辑 `/etc/config/network`，增加下面内容，并删除 `config interface 'lan'` 相关的内容：

```
config interface 'wan'
  option device 'eth0'
  option proto 'dhcp'
```

执行 `service network reload` 重启网络。

### 设置防火墙

从路由器的角度来看，WAN 口属于外部网络，自有防火墙配置大多是 REJECT 外部入口流量。所以需要修改防火墙配置。

编辑 `/etc/config/firewall`，找到 `config zone` 中 `option name wan` 的配置，将 `option input`、`option output`、`option forward` 都设置为 `ACCEPT`。

执行 `service firewall reload` 重启防火墙。

防火墙配置好后，可以使用 SSH 登录机器。可以使用 `ip addr` 查看机器的 IP 地址。

### 设置镜像

按照清华源的[说明][setup-openwrt-mirror]，执行：

```sh
sed -i 's_downloads.openwrt.org_mirrors.tuna.tsinghua.edu.cn/openwrt_' /etc/opkg/distfeeds.conf
```

[setup-openwrt-mirror]: https://mirrors.tuna.tsinghua.edu.cn/help/openwrt/

然后再执行：

```sh
opkg update
```

### 安装软件

1. samba4

```sh
opkg install samba4-server luci-app-samba4
```

2. xfs 工具

```sh
opkg install xfs-admin xfs-fsck xfs-growfs xfs-mkfs kmod-fs-xfs
```

我的数据盘全部使用 XFS 文件格式。如果选择其他文件格式，应该要安装其他工具。关于 XFS、Btrfs、EXT4 之间对比可以看这篇文章 https://zhuanlan.zhihu.com/p/348360152

3. USB 工具

```sh
opkg install block-mount blockd kmod-fs-autofs4 lsblk fdisk kmod-usb-storage kmod-usb-storage-uas
```

4. 其他工具

```sh
opkg install rsync
```

### 软件设置

软件设置就可以直接在 Luci 中进行。 

1. 挂载硬盘

菜单 -> System -> Mount Points

将相应的硬盘，然后 "Save & Apply"

2. 设置 Samba

菜单 -> Services -> Network Shares

在 Shared Directories 中增加相应的路径。如果有多个目录，Name 会设置为子目录的名称。在 Windows 上写入文件，需要选中 `Force Root`，不然会提示无权限。

