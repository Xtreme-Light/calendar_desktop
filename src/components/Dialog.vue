<template>
    <transition name="fade">
        <div class="input-box-wrapper" v-show="show">
            <div class="dialog-box" :style="`height:${dialogHeight};width:${dialogWidth};`">
                <div class="dialog-header">
                    <div class="litter-close" @click="confirm">x</div>
                </div>
                <hr />
                <div>
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
import { WatchCallback, computed, onMounted, ref, watch } from 'vue';
const props = defineProps({
    height: { type: String, required: true },
    width: { type: String, required: true },
    show: { type: Boolean, required: false }
});
// 使用computed，即计算后函数，是为了减少获取值的重复计算。使用computed可以缓存一些计算结果，另外，使用computed，依然可以将计算结果变为响应式，即原始计算数据如果是响应式的，那么computed的计算结果
// 也会是响应式的，计算结果会随着响应式数据的变化而更新。响应数据指的是ref()或则reactive()的数据，原始的js对象，基础类型并不是响应式的，所以计算结果并不会随着变化。
const dialogWidth = computed(() => (props.width ? props.width : "30%"));
const dialogHeight = computed(() => props.height ? props.height : "auto");
const show = computed(() => (props.show ? props.show : false));
const title = ref<String>("");
const content = ref("请输入事件内容");

onMounted(() => {

});
// 根据ts中的定义，提供一个限定的函数,根据WatchCallback定义，接受新值，老值，一个回调函数，改回调函数用于处理副作用
const watchShow: WatchCallback = (newShow: Boolean, oldShow: Boolean,onCleanup): void => {
    console.log("show 的值发生变化" + newShow, oldShow);
    onCleanup(()=>{
        console.log("没什么副作用");
    });
}
// 监听props中的值，发生变化时执行相应的操作
watch(() => props.show, watchShow)



const confirm = (event:Event) => {
    console.log("内容是" + event);
    console.log("获取到的" + title.value);
    console.log("获取到的" + content.value);
}

</script>
<style scoped>
 .input-box-wrapper {

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
     box-shadow: 0 5px 20px hsla(0,0%,7%,.1);
     -webkit-box-shadow: 0 5px 20px hsla(0,0%,7%,.1);
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
 ​
 .input-box-wrapper .little-close {
     color: var(--main-color);
     padding: 0 10px;
     background-color: rgb(255, 255, 255);
     box-shadow: 0px 0px 1px 2px var(--main-color);
     transition: 0.3s;
     -moz-transition: 0.3s;
     -webkit-transition: 0.3s;
 }
 ​
 .input-box-wrapper .little-close:hover {
     background-color: var(--main-color);
     color: white;
     transition: 0.3s;
     -moz-transition: 0.3s;
     -webkit-transition: 0.3s;
 }
 .dialog-box{
    display: flex;
    align-content: space-around;
    flex-direction: column;
    flex-wrap: nowrap;
    justify-content: space-between;
 }
 .dialog-btn-wrapper{
    display: flex;
    flex-direction: row-reverse;
 }
 
 </style>