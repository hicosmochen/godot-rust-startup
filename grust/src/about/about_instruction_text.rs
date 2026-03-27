use godot::prelude::*;
use godot::classes::{RichTextLabel, IRichTextLabel, Theme}; // 导入需要的 UI 类

#[derive(GodotClass)]
#[class(base=RichTextLabel)] 
pub struct AboutInstructionText {
    base: Base<RichTextLabel>
}


#[godot_api]
impl IRichTextLabel for AboutInstructionText {
    fn init(base: Base<RichTextLabel>) -> Self {
        godot_print!("使用说明"); 
        Self {base}
    }

    fn ready(&mut self) {
        let content = self.get_content();
        let mut node = self.base_mut();
        let message = format!("{}", content);
        node.append_text(&message);

        // 加载外部主题文件
        let my_theme = load::<Theme>("res://theme/main_body_label_theme.tres");
        // 应用到主节点，子节点会继承该主题
        node.set_theme(&my_theme);
    }
}

#[godot_api]
impl AboutInstructionText {
    #[func]
    fn get_content(&self) -> String {
        // 使用 r#"..."# 语法开启原始字符串
        // 里面可以自由换行、缩进，无需转义
        let content = r#"

*******************************《使用须知》*************************************

1. 功能简介：
        A. 自动创建 rust godot 项目结构
        B. 自动创建 入门案例 并且 支持运行

2. 操作前提
        您在使用的软件之前, 需要做下面的几点配置:
        A. 【环境配置】 --> 【Godot启动文件】 配置 Godot的启动文件 exe 文件。 例如: 找到 Godot 的快捷方式
        B. 【环境配置】 --> 【rust安装路径】  需要找到 .cargo 的文件夹。 例如:  C:\Users\Administrator\.cargo

3. 操作步骤
        当您做完前面的操作前提配置之后, 就可以开始 项目的创建了。您可以选择【项目创建】来创建 rust-godot 项目
        A. 第一步: 您需要选择工作空间, 工作空间是 项目所在的根目录
        B. 第二步: 您需要设置 RUST 根目录的名称, 需要注意的是 根目录的名称, 也是后期生成的 动态库dll文件的名称
        C. 第三步: 您需要设置 gdext 的名称, 该文件用于关联动态库 和 godot项目, 他是 rust 和 godot 的桥梁
        D. 第四步: 您可以选择创建案例, 来构建第一个 NodeHello 来快速入门 rust-godot  当然这个选项是非必须的
        E. 第五步: 当您操作完毕上面的所有步骤之后, 点击【开始创建项目】, 该过程将会持续一段时间, 需要您耐心等待

-----------------------------------------------------《注意事项》---------------------------------------------------------------------------------

- 1、当前脚本由开发者 cosmo 开发, 使用语言为 rust - godot 完成。
- 2、目前版本中, 由于开发者时间原因, 暂时不支持 多语言适配
- 3、目前仅在 windows 系统中完成, 其他平台未做具体测试, 可能会引起 bug 现象
- 4、如果您不够熟悉 rust-godot 代码结构的情况下, 建议您选择 【需要创建案例】
- 5、该项目代码, 已经提交 github, 需要更多支持, 请联系作者 cosmo 
- 6、当前版本的代码, 仅仅构建了 debug 版本 debug版本采用的是 cargo build
- 7、如果您有发布 release 版本的需求 
                第一: 您需要使用  cargo build --release 构建rust项目
                第二: 指定 gdext文件中 dll路径 指向 release 的路径

************************************************************************************
        "#;

        content.trim().to_string() // trim() 可以去掉首尾多余的空白换行
    }
}