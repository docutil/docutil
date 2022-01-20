# Puppeteer 实现 UI 自动化测试

Puppeteer 是~~Google 官方（可能）出品的~~用于 Chrome/Chromium 的自动动化控制库。

GitHub: https://github.com/puppeteer/puppeteer
官网：https://pptr.dev/

## 可以做什么

按我目前的理解，通过 puppeteer 可以模拟所有人工操作，调用浏览器的所有功能，甚至获取浏览器的内部数据。最典型的一些用途：

- UI 自动化测试
- 爬虫（获取数据部分）
  - 因为 puppeteer 本身是控制浏览器，因此可以处理 js动态生成的 DOM 和静态 DOM
- 打印为 PDF（浏览器自带功能）
- 网页截图（浏览器自带功能）

## puppeteer vs selenium

selenium 也是另一个控制浏览器的工具（库）。selenium 大概是 Java 生态、python 生态中最常见的浏览器控制库。selenium 通过 webdriver 接口来控制浏览器。webdriver 是 W3C 组织的一个推荐标准。

主要区别：

- 连接方式
  - selenium 通过 Restful 接口连接 webdriver，webdriver 再通过 devtools 协议连接浏览器
  - puppeteer 则直接通过 devtools 协议连接浏览器

- 开发语言支持
  - selenium 最大的优势是官方维护了 java/python/javascript/c# 几种语言的绑定
  - puppeteer 本身只是一个 node.js 模块。社区里有其他开发语言实现的基于 devtools 协议的浏览器控制库
    - golang 有 https://github.com/go-rod/rod
    - rust 有 https://crates.io/crates/chromiumoxide

另外 selenium 也提供了 Grid 这种大规模执行机支持的功能实现，也支持控制远程的 webdriver。

## puppeteer 概念

![puppeteer 概念](docs/202201/images/puppeteer_概念.svg)

## 简单的 UI 自动化测试实现 fetchweb

UI 自动化测试主要是通过浏览化的操作自动化来减少人工测试 UI 功能的投入。用 UI 自动化完全取代人工，就目前的技术情况来看，还是不现实。一是 UI 本身是强业务绑定，每个 UI 自动化的测试用例基本上是独立开发的，复用的可能性比较低；二是，对于业务还不稳定的系统，由于一的原因，必然会导致用例不断修改，这部分也是人工投入。所以 UI 自动化测试应该是一个辅助工具，代替一部分重复的劳动。

UI 自动化测试的关键是如何快速开发测试用例。最理想的工具是通过宏录制工具，录制一部分操作过程，转换为代码，最后人工再完善一下，然后成为测试用例。另一个思路则是通过用例编写用的 DSL 语言，人工编辑测试用例。fetchweb 是对第二种思路的一个尝试。（因为比较容易实现 🤣）

fetchweb 最初的版本使用 selenium 实现 chrome 浏览器控制。现在改为 puppeteer，不过这个版本里，我选用了 electron。

electron 是基于 chromium、nodejs 开发的桌面端框架。通过 electron 可以使用 web 技术栈开发跨平台的桌面软件。puppeteer 也持控制 electron。这里通过 [puppeteer-in-electron](https://www.npmjs.com/package/puppeteer-in-electron) 进行桥接。

**生产项目应该使用 puppeteer + chromium 的组合**。毕竟 electron 带有 nodejs，可能会产生不少安全问题。另外 electron 在 linux 环境中需要 Xvfb 支持，因为 electron 并不支持 chromium 的 headless 模式，这也会带来测试执行环境体积膨胀。

fetchweb 基于 yaml 定义了一个 DSL。用户使用 DSL 来描述操作过程，从而实现现 UI 的自动化操作。目前已经实现了基本的点击、获取文本、iframe 处理。DSL 是基于 YAML 进行描述，相当只有声明语法，所以没有逻辑功能。这就不利于编写复杂脚本。如果需要编写复杂脚本，直接使用 js 可能更好。

核心代码在 https://github.com/yuekcc/fetchweb/blob/main/fetchweb.js

脚本示例：

```yaml
---
name: fetch-demo-site
steps:
  # 表示开始
  - type: start
  # 进入 URL
  - type: openUrl
    url: http://localhost:10086/
  # 点击某个元素
  - type: click
    selector: # selector 总是从根窗口开始
      - window:
          src: http://localhost:10086/
      - xpath: /html/body/a
  # 在打印某个元素文本数据（到日志）
  - type: print
    selector:
      - window:
          src: http://localhost:10086/iframe-1.html
      - xpath: /html/body/iframe
      - xpath: /html/body/div
  # 断言
  - type: assert
    method: eq # 比较方法，目前只支持字符串相等（==）
    expect: 'iframe-inner-1'
    selector:
      - window:
          src: http://localhost:10086/iframe-1.html
      - xpath: /html/body/iframe
      - xpath: /html/body/div
  # 表示结束
  - type: end
```

---

2022年01月08日
