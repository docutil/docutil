# Marknote 介绍

- [在线预览](https://yuekcc.github.io/marknote/)
- [源码](https://github.com/yuekcc/marknote)

[TOC]

## 想法

Marknote 最初的想法是山寨一个 [boot-cli]([https://github.com/vvpvvp/book-cli)。

book-cli 是一个非常简单的静态网站应用。我称之为“应用”，是因为这个软件本身是一个纯前端实现的 SPA。文章的内容是 markdown 动态渲染的。book-cli 甚至支持“上一篇”/“下一篇”这样的功能。

作为一名开发，学习最好的办法就是造轮子，于是就有了 “Simple notes” 应用。名字比较随意，只是有一个称呼而已。

早期代码和文章是同一个[仓库](https://github.com/yuekcc/yuekcc.github.io)里。毕竟用户只有我自己，怎么方便就怎么写。

虽然简陋，但实现了核心的功能，算是一个 MVP 实现：

- 页面布局是左右两列，左边是导航栏，右侧是文章
- 文章内容、侧栏的内容都是通过渲染 markdown 得到
- 主页是 README.md

开发这个 MVP 大概用了半天时间。毕竟这个 SPA 的复杂度并不高。

## 新的实现

后来有朋友也想搭个 github pages，想借用我的这个 SPA 程序。当然也有主动推销的成份。有了外部用户的反馈，我也逐渐给这
个“简单笔记 App” 加上新功能。为方便更新，最终这个 App 改为独立的项目。于是就是有了 [Marknote](https://github.com/yuekcc/marknote)。

## 已实现的功能

- 页面布局改为顶部导航，下面为正文内容的布局
    - 正文部分保持左右布局，方便组织文章；左侧是导航，右则是文章
    - 文章可以指定使用不同的侧栏
    - 增加打印机优化的样式
- markdown 增加了一个简单的 `[TOC]` 语法拓展
- 主题支持大屏小屏设备
- 打印机优化

网站主页，实际上是仓库的 README.md 文件。选择这个文档为主页的目的是与 github 这样的托管服务保持一致。在 github.io 不
能访问时，可以直接访问仓库，这样网站本身起码还能看。

>github.io 域名在我用的网络偶然会抽风，访问不了

网站除主页外，应该有一个侧栏，侧栏可以起到链接、跳转的功能。这样就可以将多个文章组织为一个整体。参考一些
使用 [vuepress](https://vuepress.vuejs.org/zh/) 的文章网站，我也给文章的连接增加了 `sidebar` 参数，
用于指定渲染文章时使用不同的侧栏。在概念上，就可以实现文章分类。

顶部导航因为内容比较简单，就直接使用 html 编写。

## 后续功能计划

- [ ] front matter 支持
- [x] 完善打印样式
- [ ] ~~完善用户交互效果~~
- [ ] ~~缓存~~
- [x] pwa 支持（仅支持安装为应用）
- [x] 文档
- [x] 实现一个更好 `[TOC]` 拓展
- [x] 增加一套主题
    - ~~[ ] 支持 light/dark 切换~~
- [x] 语法高亮
- [ ] 使用 yew.rs 重写

## 用户

- https://yuekcc.github.io
- https://turn-left.github.io
- https://yuekcc.github.io/marknote/

----

- 2021.11.6：初稿
- 2021.11.7：完成部分计划


## markdown test

```rust

use yew::{function_component, html};

mod md_component;
mod util;
use md_component::*;

#[function_component(WebRoot)]
fn web_root() -> Html {
    html! {
        <div class="root"><MdView src={"/test.md"}></MdView></div>
    }
}

fn main() {
    yew::start_app::<WebRoot>();
}

```