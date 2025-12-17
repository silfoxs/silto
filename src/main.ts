import { createApp } from "vue";
import App from "./App.vue";
import "./styles/index.css";

import EditorWindow from "./EditorWindow.vue";

const urlParams = new URLSearchParams(window.location.search);
const isEditor = urlParams.get("window") === "editor";
console.log('App Main: Initializing', { isEditor, search: window.location.search });

try {
    // Switch back to real EditorWindow
    if (isEditor) {
        createApp(EditorWindow).mount("#app");
    } else {
        createApp(App).mount("#app");
    }
} catch (e: any) {
    console.error("Vue Mount Error:", e);
    // document.body.innerHTML = `<div style="color:red; padding:20px;">VUE ERROR: ${e.message}</div>`;
}
