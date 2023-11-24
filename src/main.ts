import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'

const app = createApp(App);
app.use(ElementPlus);

app.mount("#app");
app.config.errorHandler = (err) => {
    /* 处理错误 */
    console.log("出现错误", err);
}