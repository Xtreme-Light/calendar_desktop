import { defineAsyncComponent, render, createVNode, VNode } from "vue";

class Dialog {
    private component;
    private vnode:VNode|null;
    private node:ChildNode|null;
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
            show:false,
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
    public show(title: String):Promise<String> {

        const event = new CustomEvent('dialog-show',{
            detail:{
                show:true,
                title:title,
            }
        });
        this.props.show = true;
        document.dispatchEvent(event);
    
        return new Promise((resolve) => {
            document.addEventListener('dialog-confirm',(e) => {
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



const dialog = new Dialog();


export async function showDialog(title:String){
    return dialog.show(title);
}

export default function installDialog(){
    dialog.install();
}
