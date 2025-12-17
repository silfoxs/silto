import { createApp } from "vue";
import App from "./App.vue";
import "./styles/index.css";

try {
    createApp(App).mount("#app");
} catch (e: any) {
    console.error("Vue Mount Error:", e);
}
