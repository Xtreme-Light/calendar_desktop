<template>
    <transition name="fade">
        <div class="input-box-wrapper" v-show="show">
            <div class="dialog-box" :style="`height:${dialogHeight};width:${dialogWidth};`">
                <div class="dialog-header">
                    <div class="dialog-title">
                        {{ title }}
                    </div>
                    <div class="litter-close" @click="confirm('')">x</div>
                </div>
                <hr />
                <div>
                    <div style="width: 100%;">
                        <textarea v-model="content">
                        </textarea>
                    </div>
                    <div class="dialog-btn-wrapper">
                        <el-button type="primary" class="primary password-btn" @click="confirm(content)">确认</el-button>
                    </div>
                </div>
            </div>
        </div>
    </transition>
</template>
<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
const props = defineProps({
    height: { type: String, required: true },
    width: { type: String, required: true }
});
// let name = ref("Dialog");
const dialogWidth = computed(() => (props.width ? props.width : "30%"));
const dialogHeight = computed(() => props.height ? props.height : "auto");
const show = ref(false);
const title = ref("请输入内容");
const content = ref("");

onMounted(() => {
    document.addEventListener("password-dialog-show", event => {
        console.log("" + JSON.stringify(event));
        title.value = "";
    });
});


const confirm = (content: String) => {
    console.log("内容是" + content)
}

</script>