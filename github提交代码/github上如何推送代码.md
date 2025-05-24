# github上如何推送代码

两种情况：

# 一、如果是增添新的文件，给我说一下，我邀请你加入这个仓库。然后直接将这个库克隆到本地，添加新文件后可以直接提交即可：

参考二的1、2,然后......(你补充一下)  后面哪里有问题的 你根据你的情况实际修改一下。如果用图片注意图片放在了哪个的位置






# 二、 如果是**修改已有的文件**需要`PR`。也就是下面的步骤：
## 1、注册自己github账号
    自己探索
## 2、配置自己git提交信息

- [安装 git](https://blog.csdn.net/mukes/article/details/115693833)
- 设置本地 Git 账户
```git
#名字拼写就可以  邮箱设不设置都可以  有时需要添加ssh密钥什么的需要自己搜(不加ssh密钥可能每次commit都需要输入账号和密码)
git config --global user.name "你的名字" 
git config --global user.email "你的邮箱"
```
- 查看是否信息配置成功
  `git config --list`
## 3、fork要推送的项目
github地址: `https://github.com/169LI/rust-henu`

![img.png](../imageall/image/fork.png)

这里是你的仓库

![img.png](../imageall/image/fork到自己的仓库下.png)

## 4、克隆自己fork的项目到本地

![img.png](../imageall/image/Clone_http.png)

![img.png](../imageall/image/clone演示.png)

## 5、修改代码并提交到自己的仓库

![img.png](../imageall/image/修改代码.png)

如果不会处理冲突，就只添加自己要添加的文件就行，不要修改其他文件。 

这里可以使用自己ide提供的上传命令，比如 Rustrover：

![img.png](../imageall/image/Rustrover演示.png)

![img.png](../imageall/image/推送成功.png)

比如vscode（也有插件如`gitlen`或自带的`源代码管理`也可以达到同样的功能）或者 通过`命令行`使用git命令上传，自己去学习`git`命令(下面给一个简单示例)
```git
git add .
git commit -m "提交信息"
git push origin --  --
```
# 6、提交PR
![img.png](../imageall/image/PR演示.png)

![img.png](../imageall/image/PR2.png)

![img.png](../imageall/image/PR3.png)

# 7、等待审核

![img.png](../imageall/image/PR4.png)

等待我审核合并后就可以看到你的修改合并到主仓库了。

下次提交时记得先`sync`两个地方后，再添加你的代码，再`上传`和`PR`

`sync`以下两个地方：

1. sync自己fork的仓库(github)
![img.png](../imageall/image/sync.png)
2. Clone的仓库(本地)
![img.png](../imageall/image/Sync2.png)
选择 `传入更改合并到当前分支` 即可.









