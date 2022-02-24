# docutil

docutil 是一个简单快速的网站工具，适用于小型的文档网站。docutil 所有渲染都在浏览器上完成，无需后台。

## 效果

![screenshot](./screenshot.png)

[预览](https://lambdadriver.space/)

## 特性

- [x] 完全前端渲染，无后台服务
- [x] hash 风格路由
- [x] 内置主题
    - [x] 首页
    - [x] 打印优化
    - [x] 移动端支持
- [x] 全文搜索
    - [x] 独立的[搜索 API](https://github.com/yuekcc/marknote-search-mvp)
- [x] 支持自定义网站 title、footer 信息
- [x] 支持二级目录部署（https://example.com/xxx）

## 生态

- [全文搜索](https://github.com/yuekcc/marknote-search-mvp)
    - MVP 版
    - 基于 meilisearch 实现的全文搜索（[原理](https://lambdadriver.space/#/docs/202111/full-text-search-for-marknote.md)）
- [webhook](https://github.com/yuekcc/docutil-deploy-hook)
    - 用于 gitops 的 webhook 服务。通过 github 等代码托管服务的 webhook 功能，实现自有服务器的网站更新。

## License

[MIT](LICENSE)
