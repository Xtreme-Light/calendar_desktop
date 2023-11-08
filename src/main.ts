import { createApp } from "vue";
import "./styles.css";
import App from "./App.vue";

const app = createApp(App);
app.mount("#app");
app.config.errorHandler = (err) => {
    /* 处理错误 */
    console.log("出现错误", err);

}
