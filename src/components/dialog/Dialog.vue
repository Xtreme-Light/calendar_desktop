<template>
    <transition name="fade">
        <div class="input-box-wrapper" v-show="show">
            <div class="dialog-box" :style="`height:${dialogHeight};width:${dialogWidth};`">
                <div class="dialog-header">
                    <div class="dialog-title"><span>这是标题</span></div>
                    <button type="button" class="dialog-close" aria-label="Close this dialog" @click="cancel">
                        <el-icon>
                            <CircleClose />
                        </el-icon>
                    </button>
                </div>
                <hr />
                <div class="dialog-content">
                    <div>
                        <el-input type="text" v-model="title" placeholder="请输入事件标题"></el-input>
                    </div>
                    <div style="width: 100%;">
                        <el-input v-model="content" :rows="2" type="textarea" placeholder="请输入事件内容"></el-input>
                    </div>
                    <div class="dialog-btn-wrapper">
                        <el-button type="primary" class="primary password-btn" @click="confirm">确认</el-button>
                    </div>
                </div>
            </div>
        </div>
    </transition>
</template>
<script setup lang="ts">
import { computed, onMounted, ref, } from 'vue';
// 在使用ts进行runtime-only编译该vue模版时，全局导入的element-plus不生效，需要局部指定导入。
import { ElButton, ElInput, ElIcon } from 'element-plus';
import { CircleClose } from '@element-plus/icons-vue';
import { debounceRef } from '../deboundRef';


const props = defineProps({
    height: { type: String, required: true },
    width: { type: String, required: true },
});
// 使用computed，即计算后函数，是为了减少获取值的重复计算。使用computed可以缓存一些计算结果，另外，使用computed，依然可以将计算结果变为响应式，即原始计算数据如果是响应式的，那么computed的计算结果
// 也会是响应式的，计算结果会随着响应式数据的变化而更新。响应数据指的是ref()或则reactive()的数据，原始的js对象，基础类型并不是响应式的，所以计算结果并不会随着变化。
const dialogWidth = computed(() => (props.width ? props.width : "30%"));
const dialogHeight = computed(() => props.height ? props.height : "auto");
const show = ref(false);
const title = debounceRef<String>("");
const content = debounceRef<String>("请输入事件内容");


onMounted(() => {
    document.addEventListener("dialog-show", (e: Event) => {
        // 有没有别的方式来转换event 没有特别好的方式，如下方式比较好
        if (e instanceof CustomEvent) {
            if (e?.detail?.show) {
                // do something
                console.log("接收到弹窗确认事件" + JSON.stringify(e));
                show.value = e.detail.show;
            }
        }
    });
});
// 根据ts中的定义，提供一个限定的函数,根据WatchCallback定义，接受新值，老值，一个回调函数，改回调函数用于处理副作用

// 监听props中的值，发生变化时执行相应的操作

const confirm = (event: Event) => {
    console.log("内容是" + JSON.stringify(event));
    console.log("获取到的" + title.value);
    console.log("获取到的" + content.value);
    console.log("发送弹窗确认事件");
    const customEvent = new CustomEvent("dialog-confirm", {
        detail: {
            title: title.value,
            content: content.value,
        }
    });
    // 通过原生的事件机制，发布自定义事件
    document.dispatchEvent(customEvent);
    show.value = false;
}


const cancel = () => {
    console.log("用户取消弹窗");
    show.value = false;
    clear();
}

const clear = () => {
    console.log("清除内容");
    title.value = "";
    content.value = "";
}
</script>
<style scoped>
.input-box-wrapper {
    z-index: 2011;
    text-align: center;
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    padding: 16px;
    overflow: auto;
}

.dialog-box {
    display: inline-block;
    max-width: var(--el-messagebox-width);
    width: 100%;
    padding-bottom: 10px;
    vertical-align: middle;
    background-color: var(--el-bg-color);
    border-radius: var(--el-messagebox-border-radius);
    border: 1px solid var(--el-border-color-lighter);
    font-size: var(--el-messagebox-font-size);
    box-shadow: var(--el-box-shadow-light);
    text-align: left;
    overflow: hidden;
    backface-visibility: hidden;
    box-sizing: border-box;
}

.dialog-box .dialog-close {
    position: absolute;
    top: 0px;
    right: 0px;
    padding: 0;
    border: none;
    outline: none;
    background: transparent;
    font-size: 16px;
    cursor: pointer;
}

.dialog-box .dialog-header {
    position: relative;
    padding: var(--el-messagebox-padding-primary);
    padding-bottom: 10px;
}

.dialog-box .dialog-title {
    padding-left: 0;
    margin-bottom: 0;
    font-size: var(--el-messagebox-font-size);
    line-height: 1;
    color: var(--el-messagebox-title-color);
}

.dialog-content {
    padding: 10px var(--el-messagebox-padding-primary);
    color: var(--el-messagebox-content-color);
    font-size: var(--el-messagebox-content-font-size);
}

.dialog-btn-wrapper {
    padding: 5px 15px 0;
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    align-items: center;
}

/* .input-box-wrapper {

    position: fixed;
    top: 30%;
    bottom: 30%;
    left: 30%;
    right: 30%;
    background: #fff;
    backdrop-filter: var(--backFilter);
    display: flex;
    justify-content: center;
    align-items: center;
    z-index: 101;
    transition: all 0.3s linear;
    -moz-transition: all 0.3s linear;
    -webkit-transition: all 0.3s linear;
    border: 1px solid #ebebeb;
    border-radius: 4px;
    box-shadow: 0 5px 20px hsla(0, 0%, 7%, .1);
    -webkit-box-shadow: 0 5px 20px hsla(0, 0%, 7%, .1);
}

.input-box-wrapper .dialog-header {
    display: flex;
    justify-content: space-between;
}

.input-box-wrapper .little-close {
    border-radius: 0.3em;
    background-color: var(--main-color);
    color: white;
    cursor: pointer;
    padding: 0 7px;
}

.input-box-wrapper .little-close {
    color: var(--main-color);
    padding: 0 10px;
    background-color: rgb(255, 255, 255);
    box-shadow: 0px 0px 1px 2px var(--main-color);
    transition: 0.3s;
    -moz-transition: 0.3s;
    -webkit-transition: 0.3s;
}

.input-box-wrapper .little-close:hover {
    background-color: var(--main-color);
    color: white;
    transition: 0.3s;
    -moz-transition: 0.3s;
    -webkit-transition: 0.3s;
}

.dialog-box {
    display: flex;
    align-content: space-around;
    flex-direction: column;
    flex-wrap: nowrap;
    justify-content: space-between;
}

.dialog-btn-wrapper {
    display: flex;
    flex-direction: row-reverse;
} */</style>