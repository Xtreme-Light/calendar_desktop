import { defineAsyncComponent, render, createVNode, VNode } from "vue";

class Dialog {
    private component;
    private vnode: VNode | null;
    private node: ChildNode | null;
    private props;
    public constructor() {
        // https://cn.vuejs.org/guide/components/async.html
        // 异步加载组件
        this.component = defineAsyncComponent(
            () => import("./Dialog.vue")
        );
        this.vnode = null;
        this.node = null;
        this.props = {
            width: "90%",
            height: "auto",
            show: false,
            title: ""
        };
    }
    public install() {
        if (!this.vnode) {
            // 初始化组件生成vdom
            const dialog = createVNode(this.component, this.props);
            const container = document.createElement('div');
            //render通过patch 变成dom
            render(dialog, container);

            this.vnode = dialog;
            this.node = container.childNodes[0];

            document.body.appendChild(this.node);
        }
    }
    public show(data: DialogDTO): Promise<String> {

        const event = new CustomEvent('dialog-show', {
            detail: {
                show: true,
                title: data?.title,
                width: data?.width,
                height: data?.height
            }
        });
        this.props.show = true;
        document.dispatchEvent(event);

        return new Promise((resolve) => {
            document.addEventListener('dialog-confirm', (e) => {
                if (e instanceof CustomEvent) {
                    if (e?.detail?.content) {
                        // do something
                        console.log("接收到弹窗确认事件" + JSON.stringify(e));
                        resolve(e.detail.content);
                    }
                }
            });
        });
    }
}
// 定义一个ts的类
export class DialogDTO {
    public title: String;
    public content: String | null;
    public width: String | null | undefined;
    public height: String | null | undefined;
    public constructor(title: String, content: String, width?: String, height?: String) {
        this.title = title;
        this.content = content;
        if (width) {
            this.width = width;
        }
        if (height) {
            this.height = height;
        }
    }

}


const dialog = new Dialog();


/**
 * 显示对话框
 * @param data 对话框数据
 * @returns Promise 包含显示对话框的结果
 */
export async function showDialog(data: DialogDTO) {
    return dialog.show(data);
}

/**
 * 默认导出函数，用于安装对话框
 */
export default function installDialog() {
    dialog.install();
}

