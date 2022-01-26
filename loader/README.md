# marknote

marknote 是 pure-frontend 动态网站。动态是指页面的内容由程序动态渲染。

marknote 会使用 fetch api 下载 markdown 文档，然后进行渲染。

marknote 带有一个 hash 路由。首页是 README.md，侧栏是 SIDEBAR.md。markdown 文档中的非绝对路径将会自动转换为 `/#/path/doc.md` 链接。