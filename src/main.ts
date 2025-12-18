import { createApp } from "vue";
import App from "./App.vue";
import "./styles/index.css";
import i18n from "./i18n";

try {
    const app = createApp(App);
    app.use(i18n);
    app.mount("#app");
} catch (e: any) {
    console.error("Vue Mount Error:", e);
}
